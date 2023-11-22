use libc::{timespec, clock_gettime, CLOCK_BOOTTIME};
use std::mem::MaybeUninit;

use crate::config::UNKNOWNSTR;

pub fn uptime(_: &str) -> String {
    let mut buffer = MaybeUninit::<timespec>::uninit();
    if unsafe { clock_gettime(CLOCK_BOOTTIME, buffer.as_mut_ptr()) } < 0 {
        return UNKNOWNSTR.to_owned();
    }

    let uptime = unsafe { buffer.assume_init() };
    let h: i64 = uptime.tv_sec / 3600;
    let m: i64 = uptime.tv_sec % 3600 / 60;

    return format!("{}h {}m", h, m);
}
