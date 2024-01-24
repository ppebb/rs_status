use libc::statvfs;
use std::{ffi::CString, mem::MaybeUninit};

use crate::util::format_human;

pub fn statvfs_wrapper(path: &str) -> Result<statvfs, String> {
    let mut buffer = MaybeUninit::<statvfs>::uninit();
    let cpath = CString::new(path).unwrap();

    unsafe {
        let ret = statvfs(cpath.as_ptr(), buffer.as_mut_ptr());

        if ret < 0 {
            return Err(format!("unable to statvfs {}", path));
        }
        return Ok(buffer.assume_init());
    }
}

pub fn disk_free(path: &str) -> String {
    return match statvfs_wrapper(path) {
        Ok(fs) => format_human(fs.f_frsize * fs.f_bavail, 1024),
        Err(e) => e,
    };
}

pub fn disk_perc(path: &str) -> String {
    return match statvfs_wrapper(path) {
        Ok(fs) => ((100.0 * (1.0 - (fs.f_bavail as f32 / fs.f_blocks as f32))) as i64).to_string(),
        Err(e) => e,
    };
}

pub fn disk_total(path: &str) -> String {
    return match statvfs_wrapper(path) {
        Ok(fs) => format_human(fs.f_frsize * fs.f_blocks, 1024),
        Err(e) => e,
    };
}

pub fn disk_used(path: &str) -> String {
    return match statvfs_wrapper(path) {
        Ok(fs) => format_human(fs.f_frsize * (fs.f_blocks - fs.f_bfree), 1024),
        Err(e) => e,
    };
}
