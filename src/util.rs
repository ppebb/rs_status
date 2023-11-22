use std::{io::{BufReader, Read}, fs};

pub fn lines(path: &str) -> String {
    let proc_stat = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read /proc/stat")
    };
    let mut buffer = BufReader::new(proc_stat);
    let mut lines = String::new();
    let _ = buffer.read_to_string(&mut lines);

    return lines
}

pub fn first_line(path: &str) -> String {
    let lines = lines(path);

    return lines.lines().nth(0).unwrap().to_owned();
}
