use scan_fmt::scan_fmt;

use crate::{config, util::first_line};

static mut O: [i64; 7] = [0; 7];
pub fn cpu_perc(_: &str) -> String {
    let mut p: [i64; 7] = [0; 7];
    (p[0], p[1], p[2], p[3], p[4], p[5], p[6]) = scan_fmt!(&first_line("/proc/stat"), "cpu{/[\\s]*(\\d)/} {d} {d} {d} {d} {d} {d}", i64, i64, i64, i64, i64, i64, i64).unwrap();

    // I really shouldn't need to mark this unsafe because I'm never going to be accessing this
    // from multiple threads, but oh well
    unsafe {
        let sum = (O[0] + O[1] + O[2] + O[3] + O[4] + O[5] + O[6]) -
            (p[0] + p[1] + p[2] + p[3] + p[4] + p[5] + p[6]);

        if sum == 0 {
            O = p;
            return config::UNKNOWNSTR.to_owned();
        }

        let avg = 100 *
            ((O[0] + O[1] + O[2] + O[5] + O[6]) -
             (p[0] + p[1] + p[2] + p[5] + p[6])) / sum;

        O = p;
        return avg.to_string();
    }
}
