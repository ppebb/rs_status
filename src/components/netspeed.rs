use scan_fmt::scan_fmt;

use crate::{
    config,
    util::{format_human, lines},
};

pub fn netspeed_rx(interface: &str) -> String {
    static mut OLDRXBYTES: u64 = 0;
    let path = format!("/sys/class/net/{}/statistics/rx_bytes", interface);

    let rxbytes = scan_fmt!(&lines(&path), "{d}", u64).unwrap();

    let speed: u64;
    unsafe {
        speed = (rxbytes - OLDRXBYTES) * 1000 / config::INTERVAL;
        OLDRXBYTES = rxbytes;
    }

    return format_human(speed, 1024);
}

pub fn netspeed_tx(interface: &str) -> String {
    static mut OLDTXBYTES: u64 = 0;
    let path = format!("/sys/class/net/{}/statistics/tx_bytes", interface);

    let txbytes = scan_fmt!(&lines(&path), "{d}", u64).unwrap();

    let speed: u64;
    unsafe {
        speed = (txbytes - OLDTXBYTES) * 1000 / config::INTERVAL;
        OLDTXBYTES = txbytes;
    }

    return format_human(speed, 1024);
}
