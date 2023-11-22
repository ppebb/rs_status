use chrono::Local;

pub fn datetime(format: &str) -> String {
    let date = Local::now();

    return date.format(format).to_string();
}
