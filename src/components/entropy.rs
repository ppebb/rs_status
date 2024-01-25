use scan_fmt::scan_fmt;

use crate::util::lines;

pub fn entropy(_: &str) -> String {
    return match scan_fmt!(&lines("/proc/sys/kernel/random/entropy_avail"), "{d}", u64) {
        Ok(val) => val.to_string(),
        Err(e) => e.to_string(),
    };
}
