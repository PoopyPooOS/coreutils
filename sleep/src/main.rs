use std::{env, process, thread, time::Duration};

#[derive(Debug)]
enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
}

fn main() {
    let unit_text = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sleep <time>");
        process::exit(1);
    });

    let unit = match unit_text {
        _ if unit_text.ends_with("s") => TimeUnit::Second,
        _ if unit_text.ends_with("m") => TimeUnit::Minute,
        _ if unit_text.ends_with("h") => TimeUnit::Hour,
        _ if unit_text.ends_with("d") => TimeUnit::Day,
        _ => TimeUnit::Second,
    };

    let unit_num = unit_text.trim_matches(|c: char| !c.is_ascii_digit()).parse().unwrap();

    match (unit_num, unit) {
        (s, TimeUnit::Second) => thread::sleep(Duration::from_secs(s)),
        (s, TimeUnit::Minute) => thread::sleep(Duration::from_secs(s)),
        (s, TimeUnit::Hour) => thread::sleep(Duration::from_secs(s)),
        (s, TimeUnit::Day) => thread::sleep(Duration::from_secs(s)),
    }
}
