#include "build_info.h"
#include <zero/log.h>
#include <zero/proc/process.h>
#include <zero/filesystem/path.h>
#include <regex>
#include <sys/user.h>
#include <unistd.h>

constexpr auto GO_BUILD_INFO = "buildinfo";
constexpr auto GO_BUILD_INFO_MAGIC = "\xff Go buildinf:";
constexpr auto GO_BUILD_INFO_OFFSET = 16;

constexpr auto GO_REGISTER_BASED_MAJOR = 1;
constexpr auto GO_REGISTER_BASED_MINOR = 17;

constexpr auto POINTER_FREE_FLAG = 0x2;
constexpr auto POINTER_FREE_OFFSET = 32;

constexpr auto MAX_VAR_INT_LENGTH = 10;

static bool readUVarInt(const unsigned char **pp, unsigned long &value) {
    unsigned int v = 0;
    unsigned int shift = 0;

    const unsigned char *p = *pp;

    for (int i = 0; i < MAX_VAR_INT_LENGTH; i++) {
        unsigned long b = *p++;

        if (b < 0x80) {
            if (i == MAX_VAR_INT_LENGTH - 1 && b > 1)
                return false;

            *pp = p;
            value = v | b << shift;

            return true;
        }

        v |= (b & 0x7f) << shift;
        shift += 7;
    }

    return false;
}

bool CBuildInfo::load(const std::string &file) {
    zero::proc::CProcessMapping processMapping;

    if (!zero::proc::getImageBase(getpid(), file, processMapping))
        return false;

    return load(file, processMapping.start);
}

bool CBuildInfo::load(const std::string &file, unsigned long base) {
    ELFIO::elfio reader;

    if (!reader.load(file)) {
        LOG_ERROR("open elf failed: %s", file.c_str());
        return false;
    }

    auto it = std::find_if(
            reader.sections.begin(),
            reader.sections.end(),
            [](const auto& s) {
                return zero::strings::containsIC(s->get_name(), GO_BUILD_INFO);
            });

    if (it == reader.sections.end()) {
        LOG_ERROR("can't find go build info section");
        return false;
    }

    char *data = (char *)(*it)->get_address();
    size_t magicSize = strlen(GO_BUILD_INFO_MAGIC);

    if (reader.get_type() != ET_EXEC) {
        std::vector<ELFIO::segment *> loads;

        std::copy_if(
                reader.segments.begin(),
                reader.segments.end(),
                std::back_inserter(loads),
                [](const auto &i){
                    return i->get_type() == PT_LOAD;
                });

        auto minElement = std::min_element(
                loads.begin(),
                loads.end(),
                [](const auto &i, const auto &j) {
                    return i->get_virtual_address() < j->get_virtual_address();
                });

        data += base - ((*minElement)->get_virtual_address() & ~(PAGE_SIZE - 1));
    }

    if (memcmp(data, GO_BUILD_INFO_MAGIC, magicSize) != 0) {
        LOG_ERROR("go build info magic error");
        return false;
    }

    mPtrSize = (unsigned char)data[magicSize];
    mEndian = (go::endian)data[magicSize + 1];

    go::string buildVersion = {};
    go::string modInfo = {};

    if (mEndian & POINTER_FREE_FLAG) {
        mEndian = (go::endian)(mEndian & ~POINTER_FREE_FLAG);

        unsigned long length = 0;
        char *p = &data[POINTER_FREE_OFFSET];

        if (!readUVarInt((const unsigned char **)&p, length))
            return false;

        buildVersion = {p, (ptrdiff_t)length};
        p += length;

        if (!readUVarInt((const unsigned char **)&p, length))
            return false;

        modInfo = {p, (ptrdiff_t)length};
    } else {
        buildVersion = **(go::string **)&data[GO_BUILD_INFO_OFFSET];
        modInfo = **(go::string **)&data[GO_BUILD_INFO_OFFSET + mPtrSize];
    }

    mVersion = buildVersion.toSTDString();

    std::smatch match;

    if (!std::regex_match(mVersion, match, std::regex(R"(^go(\d+)\.(\d+).*)")))
        return false;

    unsigned long major = 0;
    unsigned long minor = 0;

    if (zero::strings::toNumber(match.str(1), major) && zero::strings::toNumber(match.str(2), minor)) {
        mRegisterBased = major > GO_REGISTER_BASED_MAJOR || (major == GO_REGISTER_BASED_MAJOR && minor >= GO_REGISTER_BASED_MINOR);
    }

    if (modInfo.empty()) {
        LOG_INFO("module info empty");
        return true;
    }

    return readModuleInfo(modInfo);
}

bool CBuildInfo::load() {
    return load(zero::filesystem::path::getApplicationPath());
}

bool CBuildInfo::readModuleInfo(const go::string &modInfo) {
    if (modInfo.length < 32) {
        LOG_ERROR("module info invalid");
        return false;
    }

    std::string info(modInfo.data + 16, modInfo.length - 32);
    std::vector<std::string> mods = zero::strings::split(info, "\n");

    auto readEntry = [](const std::string &m, CModule &module) {
        std::vector<std::string> tokens = zero::strings::split(m, "\t");

        if (tokens.size() != 4)
            return false;

        module.path = tokens[1];
        module.version = tokens[2];
        module.sum = tokens[3];

        return true;
    };

    for (const auto &m: mods) {
        if (zero::strings::startsWith(m, "path")) {
            std::vector<std::string> tokens = zero::strings::split(m, "\t");

            if (tokens.size() != 2)
                continue;

            mModuleInfo.path = tokens[1];
        } else if (zero::strings::startsWith(m, "mod")) {
            CModule module = {};

            if (!readEntry(m, module))
                continue;

            mModuleInfo.main = module;
        } else if (zero::strings::startsWith(m, "dep")) {
            CModule module = {};

            if (!readEntry(m, module))
                continue;

            mModuleInfo.deps.push_back(module);
        } else if (zero::strings::startsWith(m, "=>")) {
            CModule module = {};

            if (!readEntry(m, module))
                continue;

            mModuleInfo.deps.back().replace = new CModule(module);
        }
    }

    return true;
}
