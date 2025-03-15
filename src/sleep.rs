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
        _ if time.ends_with('s') | time.ends_with("sec") => TimeUnit::Second,
        _ if time.ends_with('m') | time.ends_with("min") => TimeUnit::Minute,
        _ if time.ends_with('h') | time.ends_with("hour") | time.ends_with("hr") => TimeUnit::Hour,
        _ if time.ends_with('d') | time.ends_with("days") => TimeUnit::Day,
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
