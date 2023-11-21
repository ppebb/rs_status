use std::{io::{BufReader, BufRead}, fs};

use crate::config;

static mut O: [i64; 7] = [0; 7];
pub fn cpu_perc(_: &str) -> String {
    let proc_stat = match fs::File::open("/proc/stat") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read /proc/stat")
    };
    let mut buffer = BufReader::new(proc_stat);
    let mut first_line = String::new();
    let _ = buffer.read_line(&mut first_line);

    let split = first_line.split_whitespace();

    let mut i: usize = 0;
    let mut p: [i64; 7] = [0; 7];
    for chunk in split {
        if chunk.contains("cpu") || i > 6 {
            continue
        }
        else {
            p[i] = chunk.parse().unwrap();
            i += 1;
        }
    }

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
