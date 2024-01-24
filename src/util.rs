use std::{io::{BufReader, Read}, fs, ffi::CString, os::raw::c_char};
use libc::{snprintf, access};

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

pub fn snprintf_wrapper(format: &str, args: &[String]) -> String {
    let mut buf = vec![0u8; 0];
    let cformat = CString::new(format).unwrap();
    let clen: usize = unsafe { // This will spit out the correct size of the buffer so we don't
        // need to just pray the resulting string was under some fixed value
        snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            cformat.as_ptr(),
            args.len(),
            args.as_ptr()
        )
    }
    .try_into()
    .unwrap();

    buf = vec![0u8; clen + 1];
    let clen: usize = unsafe { // The actual string!!
        snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            cformat.as_ptr(),
            args.len(),
            args.as_ptr()
        )
    }
    .try_into()
    .unwrap();

    buf.truncate(clen);
    return String::from_utf8(buf).unwrap();
}

pub fn access_wrapper(name: &str, amode: i32) -> bool {
    return unsafe {
        access(CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const i8, amode) != 0
    }
}
