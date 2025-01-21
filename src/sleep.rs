use clap::Parser;
use std::{io, thread, time::Duration};

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

    let length = time
        .trim_matches(|c: char| !c.is_ascii_digit())
        .parse()
        .expect("Failed to parse time length");

    thread::sleep(match (length, unit) {
        (s, TimeUnit::Second) => Duration::from_secs(s),
        (s, TimeUnit::Minute) => Duration::from_mins(s),
        (s, TimeUnit::Hour) => Duration::from_hours(s),
        (s, TimeUnit::Day) => Duration::from_days(s),
    });

    Ok(())
}
