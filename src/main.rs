use libc::snprintf;
use std::{convert::TryInto, env, ffi::CString, os::raw::c_char};
use x11rb::{connection::Connection, wrapper::ConnectionExt, protocol::xproto::{PropMode, AtomEnum}};

mod config;
mod components;
mod util;

fn snprintf_wrapper(format: &str, arg: String) -> String {
    let mut buf = vec![0u8; 0];
    let cformat = CString::new(format).unwrap();
    let carg = CString::new(arg).unwrap();
    let clen: usize = unsafe { // This will spit out the correct size of the buffer so we don't
        // need to just pray the resulting string was under some fixed value
        snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            cformat.as_ptr(),
            carg.as_ptr()
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
            carg.as_ptr()
        )
    }
    .try_into()
    .unwrap();

    buf.truncate(clen);
    return String::from_utf8(buf).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdout = false;
    let mut once= false;

    for arg in args {
        match arg.as_str() {
            "-s" => stdout = true,
            "-1" => once = true,
            &_ => continue
        }
    }

    let components = config::get_components();
    let interval = std::time::Duration::from_millis(config::INTERVAL);
    let mut status: Vec<String> = vec![String::new(); components.len()];

    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    loop {
        let start = std::time::Instant::now();

        for i in 0..components.len() {
            let component = &components[i];
            status[i] = snprintf_wrapper(component.format, (component.func)(component.args));
        }

        let status_str: String = status.join("");

        if stdout {
            println!("{}", status_str);
        }
        else {
            let _ = conn.change_property8(PropMode::REPLACE, root, AtomEnum::WM_NAME, AtomEnum::STRING, status_str.as_bytes());
            let _ = conn.flush();
        }

        if once {
            break;
        }

        std::thread::sleep(interval - start.elapsed());
    }
}
