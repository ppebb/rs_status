#![allow(dead_code)]

use std::env;
use util::snprintf_wrapper;
use x11rb::{
    connection::Connection,
    protocol::xproto::{AtomEnum, PropMode},
    wrapper::ConnectionExt,
};

mod components;
mod config;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdout = false;
    let mut once = false;

    for arg in args {
        match arg.as_str() {
            "-s" => stdout = true,
            "-1" => once = true,
            &_ => continue,
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
        } else {
            let _ = conn.change_property8(
                PropMode::REPLACE,
                root,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                status_str.as_bytes(),
            );
            let _ = conn.flush();
        }

        if once {
            break;
        }

        std::thread::sleep(interval - start.elapsed());
    }
}
