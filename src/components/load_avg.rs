use libc::getloadavg;

pub fn getloadavg_wrapper() -> Result<[f64; 3], String> {
    let mut ret = [0f64; 3];

    unsafe {
        let err = getloadavg(ret.as_mut_ptr(), ret.len() as i32);

        if err < 0 {
            return Err("getloadavg failed".to_owned());
        }

        return Ok(ret);
    }
}

pub fn load_avg(_: &str) -> String {
    return match getloadavg_wrapper() {
        Ok(avg) => format!("{:.2} {:.2} {:.2}", avg[0], avg[1], avg[2]),
        Err(e) => e,
    };
}
