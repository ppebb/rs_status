use std::os::raw::c_char;

use libc::gethostname;

use crate::util::string_from_u8_nul_utf8;

fn gethostname_wrapper() -> Result<String, String> {
    let mut buf = vec![0u8; 1024];

    unsafe {
        let ret = gethostname(buf.as_mut_ptr() as *mut c_char, buf.len());

        if ret < 0 {
            return Err(String::from("unable to gethostname"));
        }

        // This kinda sucks...
        return match string_from_u8_nul_utf8(&mut buf) {
            Ok(ret) => Ok(ret.to_owned()),
            Err(e) => Err(e.to_string()),
        };
    }
}

pub fn hostname(_: &str) -> String {
    return match gethostname_wrapper() {
        Ok(name) => name,
        Err(e) => e,
    };
}
