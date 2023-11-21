use crate::components::{run_command::run_command, cpu::cpu_perc};


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

    push_arg!(vec, run_command, "^c#11111b^^b#a6e3a1^[%s]", "npspotify");
    push_arg!(vec, run_command, "^b#89b4fa^[VOL: %s%%]", "pamixer --get-volume");
    push_arg!(vec, cpu_perc, "^b#f38ba8^[CPU: %s%%]", ""); // Is none.unwrap even valid
    return vec;
}
