use std::mem::MaybeUninit;

use libc::{uname, utsname};

fn uname_wrapper() -> Result<utsname, String> {
    let mut buf = MaybeUninit::<utsname>::uninit();

    unsafe {
        let ret = uname(buf.as_mut_ptr());

        if ret < 0 {
            return Err("unable to retreive uname".to_owned());
        }

        return Ok(buf.assume_init());
    }
}

pub fn kernel_release(_: &str) -> String {
    return match uname_wrapper() {
        Ok(utsname) => format!("{:?}", utsname.release),
        Err(e) => e,
    };
}
