use scan_fmt::scan_fmt;

use crate::{
    config::UNKNOWNSTR,
    util::{first_line, format_human, lines},
};

pub fn ram_free(_: &str) -> String {
    const PATTERN: &str = "MemTotal:{/[\\s]*(\\d)+/} kB
MemFree:{/[\\s]*(\\d)+/} kB
MemAvailable:{/[\\s]*(\\d)+/} kB
    ";

    let (_, _, free) = scan_fmt!(&lines("/proc/meminfo"), PATTERN, u64, u64, u64).unwrap();

    return format_human(free * 1024, 1024);
}

pub fn ram_perc(_: &str) -> String {
    const PATTERN: &str = "MemTotal:{/[\\s]*(\\d)+/} kB
MemFree:{/[\\s]*(\\d)+/} kB
MemAvailable:{/[\\s]*(\\d)+/} kB
Buffers:{/[\\s]*(\\d)+/} kB
Cached:{/[\\s]*(\\d)+/} kB
    ";

    let (total, free, _, buffers, cached) =
        scan_fmt!(&lines("/proc/meminfo"), PATTERN, u64, u64, u64, u64, u64).unwrap();

    if total == 0 {
        return UNKNOWNSTR.to_owned();
    }

    return (100 * ((total - free) - (buffers + cached)) / total).to_string();
}

pub fn ram_total(_: &str) -> String {
    let total = scan_fmt!(&first_line("/proc/meminfo"), "MemTotal: {d} kB\n", u64).unwrap();

    return format_human(total * 1024, 1024);
}

pub fn ram_used(_: &str) -> String {
    const PATTERN: &str = "MemTotal:{/[\\s]*(\\d)+/} kB
MemFree:{/[\\s]*(\\d)+/} kB
MemAvailable:{/[\\s]*(\\d)+/} kB
Buffers:{/[\\s]*(\\d)+/} kB
Cached:{/[\\s]*(\\d)+/} kB
    ";

    let (total, free, _, buffers, cached) =
        scan_fmt!(&lines("/proc/meminfo"), PATTERN, u64, u64, u64, u64, u64).unwrap();

    return format_human((total - free - buffers - cached) * 1024, 1024);
}
