use scan_fmt::scan_fmt;

use crate::util::{access_wrapper, lines, snprintf_wrapper};

fn pick(bat: &str, f1: &str, f2: &str) -> Result<String, String> {
    let path = snprintf_wrapper(f1, &[bat.to_owned()]);

    match access_wrapper(&path, 4) {
        true => return Ok(path.clone()),
        false => {
            let path2 = snprintf_wrapper(f2, &[bat.to_owned()]);

            match access_wrapper(&path, 4) {
                true => return Ok(path.clone()),
                false => {
                    return Err(format!("failed to find battery {} or {}", path, path2));
                }
            }
        }
    }
}

pub fn battery_perc(bat: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/capacity", bat);

    return match scan_fmt!(&lines(&path), "{d}", i32) {
        Ok(perc) => format!("{}", perc),
        Err(_) => String::new(),
    };
}

pub fn battery_state(bat: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/status", bat);

    return match &lines(&path)[..12] {
        "Charging" => "+".to_owned(),
        "Discharging" => "-".to_owned(),
        "Full" => "o".to_owned(),
        &_ => "?".to_owned(),
    };
}

pub fn battery_remaining(bat: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/status", bat);

    let charge_now: u64 = match pick(
        bat,
        "/sys/class/power_supply/%s/charge_now",
        "/sys/class/power_supply/%s/energy_now",
    ) {
        Ok(path) => scan_fmt!(&lines(&path), "{d}", u64).unwrap(),
        Err(e_string) => return e_string,
    };

    let state = &lines(&path)[..12];

    if state == "Discharging" {
        return match pick(
            bat,
            "/sys/class/power_supply/%s/current_now",
            "/sys/class/power_supply/%s/power_now",
        ) {
            Ok(path) => {
                let current_now: u64 = scan_fmt!(&lines(&path), "{d}", u64).unwrap();

                if current_now == 0 {
                    return String::new();
                }

                let timeleft = charge_now as f64 / current_now as f64;
                let h: u64 = timeleft as u64;
                let m: u64 = ((timeleft - h as f64) * 60.0) as u64;

                return format!("{}h {}m", h, m);
            }
            Err(e_string) => e_string,
        };
    }

    return String::new();
}
