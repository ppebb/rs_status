use std::{io::{BufReader, Read}, fs};

pub fn format_human(num: u64, base: u64) -> String {
    let prefix;

    let prefix_1000 = [ "", "k", "M", "G", "T", "P", "E", "Z", "Y" ];
    let prefix_1024 = [ "", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi" ];

    match base {
        1000 => prefix = prefix_1000,
        1024 => prefix = prefix_1024,
        _ => panic!("wrong prefix")
    }

    let fbase = base as f32;
    let mut scaled: f32 = num as f32;
    let mut j = 0;
    for i in 0..prefix.len() {
        j = i;

        if scaled <= fbase {
            break;
        }

        scaled /= fbase ;
    }

    return format!("{:.1} {}", scaled, prefix[j])
}

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
