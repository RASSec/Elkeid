use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    pub static ref SCHEMA: HashMap<u32, &'static [&'static [u8]]> = {
        let mut map = HashMap::<u32, &[&[u8]]>::new();
        map.insert(
            2,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"flags",
                b"mode",
                b"file",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            10,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"mprotect_prot",
                b"owner_pid",
                b"owner_file",
                b"vm_file",
                b"pid_tree",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
            ],
        );
        map.insert(
            35,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"sec",
                b"nsec",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            42,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"sa_family",
                b"dip",
                b"dport",
                b"sip",
                b"sport",
                b"res",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            43,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"sa_family",
                b"dip",
                b"dport",
                b"sip",
                b"sport",
                b"res",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            49,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"sa_family",
                b"sip",
                b"sport",
                b"res",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            59,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"argv",
                b"run_path",
                b"stdin",
                b"stdout",
                b"dip",
                b"dport",
                b"sip",
                b"sport",
                b"sa_family",
                b"pid_tree",
                b"tty",
                b"socket_pid",
                b"ssh",
                b"ld_preload",
                b"res",
                b"socket_argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
            ],
        );
        map.insert(
            60,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            62,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"target_pid",
                b"sig",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
                b"target_argv",
            ],
        );
        map.insert(
            82,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"old_name",
                b"new_name",
                b"sb_id",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            86,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"old_name",
                b"new_name",
                b"sb_id",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            101,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"ptrace_request",
                b"target_pid",
                b"addr",
                b"data",
                b"pid_tree",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"target_argv",
            ],
        );
        map.insert(
            112,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            157,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"option",
                b"new_name",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            165,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"pid_tree",
                b"dev",
                b"file_path",
                b"fstype",
                b"flags",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
            ],
        );
        map.insert(
            200,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"target_pid",
                b"sig",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            231,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            356,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"fd_name",
                b"flags",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            601,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"query",
                b"sa_family",
                b"dip",
                b"dport",
                b"sip",
                b"sport",
                b"opcode",
                b"rcode",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            602,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"file_path",
                b"dip",
                b"dport",
                b"sip",
                b"sport",
                b"sa_family",
                b"socket_pid",
                b"sb_id",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
                b"socket_argv",
            ],
        );
        map.insert(
            603,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"ko_file",
                b"pid_tree",
                b"run_path",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
            ],
        );
        map.insert(
            604,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"pid_tree",
                b"old_uid",
                b"res",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"old_username",
            ],
        );
        map.insert(
            605,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"file",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            606,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"file",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(607, &[b"exe", b"argv", b"wait"]);
        map.insert(
            608,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"file",
                b"sb_id",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            609,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"file",
                b"sb_id",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            610,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"product_info",
                b"manufacturer",
                b"serial",
                b"action",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
            ],
        );
        map.insert(
            611,
            &[
                b"uid",
                b"exe",
                b"pid",
                b"ppid",
                b"pgid",
                b"tgid",
                b"sid",
                b"comm",
                b"nodename",
                b"sessionid",
                b"pns",
                b"root_pns",
                b"dpid",
                b"pid_tree",
                b"dcred",
                b"cred",
                b"argv",
                b"ppid_argv",
                b"pgid_argv",
                b"username",
                b"pod_name",
                b"exe_hash",
                b"pid_tree",
                b"dpid_argv",
            ],
        );
        map.insert(700, &[b"module_name"]);
        map.insert(701, &[b"module_name", b"syscall_number"]);
        map.insert(702, &[b"module_name"]);
        map.insert(703, &[b"module_name", b"interrupt_number"]);
        map
    };
}
