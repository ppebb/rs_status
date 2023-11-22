use scan_fmt::scan_fmt;

use crate::{util::lines, config::UNKNOWNSTR};

static PATTERN: &str = "MemTotal:{/[\\s]*(\\d)+/} kB
MemFree:{/[\\s]*(\\d)+/} kB
MemAvailable:{/[\\s]*(\\d)+/} kB
Buffers:{/[\\s]*(\\d)+/} kB
Cached:{/[\\s]*(\\d)+/} kB
";

pub fn ram_perc(_: &str) -> String {
    let (total, free, _, buffers, cached) = scan_fmt!(&lines("/proc/meminfo"), PATTERN, u64, u64, u64, u64, u64).unwrap();

    if total == 0 {
        return UNKNOWNSTR.to_owned();
    }

    return (100 * ((total - free) - (buffers + cached)) / total).to_string();
}
