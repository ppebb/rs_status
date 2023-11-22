use libc::statvfs;
use std::{mem::MaybeUninit, ffi::CString};

pub fn statvfs_wrapper(path: &str) -> statvfs {
    let mut buffer = MaybeUninit::<statvfs>::uninit();
    let cpath = CString::new(path).unwrap();

    unsafe {
        statvfs(cpath.as_ptr(), buffer.as_mut_ptr());
        return buffer.assume_init();
    }
}

pub fn disk_perc(path: &str) -> String {
    let fs = statvfs_wrapper(path);

    return ((100.0 * (1.0 - (fs.f_bavail as f32 / fs.f_blocks as f32))) as i64).to_string();
}
