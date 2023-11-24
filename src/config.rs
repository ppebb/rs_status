#[allow(unused_imports)]
use crate::components::{run_command::run_command, cpu::{cpu_perc, cpu_freq}, ram::{ram_free, ram_perc}, disk::disk_perc, uptime::uptime, datetime::datetime};

macro_rules! push_arg {
    ( $vec:expr, $func:expr, $format:expr, $args:expr) => {
        $vec.push(Component { func: $func, format: $format, args: $args })
    };
}

// Text to show if no value can be retreived
pub const UNKNOWNSTR: &str = "n/a";

// Update interval in ms
pub const INTERVAL: u64 = 500;

type Cmd = fn(&str) -> String;

pub struct Component<'a> {
    pub func: Cmd,
    pub format: &'a str,
    pub args: &'a str
}

pub fn get_components() -> Vec<Component<'static>> {
    let mut vec: Vec<Component> = Vec::new();

    // push_arg!(vec, run_command, "^c#11111b^^b#a6e3a1^[%s]", "npspotify");
    // push_arg!(vec, run_command, "^b#89b4fa^[VOL: %s%%]", "pamixer --get-volume");
    // push_arg!(vec, cpu_perc, "^b#f38ba8^[CPU: %s%%]", "");
    // push_arg!(vec, ram_perc, "^b#eba0ac^[RAM: %s%%]", "");
    // push_arg!(vec, disk_perc, "^b#eba0ac^[DISK: %s%%]", "/");
    // push_arg!(vec, run_command, "^b#74c7ec^[PKG: %s]" , "pacman -Q | wc -l");
    // push_arg!(vec, run_command, "^b#fab387^[TEMP: %s]", "sensors | awk '/^Tctl/ {print $2}'");
    // push_arg!(vec, uptime, "^b#94e2d5^[UP: %s]", "");
    // push_arg!(vec, datetime, "^b#f5c2e7^[%s]", "%a %b %d %r");
    push_arg!(vec, cpu_freq, "%s", "");
    push_arg!(vec, ram_free, "%s", "");
    return vec;
}
