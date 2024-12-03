use std::{io, thread, time::Duration};

use clap::Parser;

#[derive(Debug)]
enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(required = true)]
    time: String,
}

pub fn sleep(args: impl Iterator<Item = String>) -> io::Result<()> {
    let Cli { time } = Cli::parse_from(args);

    let unit = match time {
        _ if time.ends_with('s') => TimeUnit::Second,
        _ if time.ends_with('m') => TimeUnit::Minute,
        _ if time.ends_with('h') => TimeUnit::Hour,
        _ if time.ends_with('d') => TimeUnit::Day,
        _ => TimeUnit::Second,
    };

    let unit_num = time.trim_matches(|c: char| !c.is_ascii_digit()).parse().unwrap();

    match (unit_num, unit) {
        (s, TimeUnit::Second) => thread::sleep(Duration::from_secs(s)),
        (s, TimeUnit::Minute) => thread::sleep(Duration::from_mins(s)),
        (s, TimeUnit::Hour) => thread::sleep(Duration::from_hours(s)),
        (s, TimeUnit::Day) => thread::sleep(Duration::from_days(s)),
    }

    Ok(())
}
