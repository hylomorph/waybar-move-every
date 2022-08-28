use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use clap::Parser;
use core::time;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;
use std::thread::{self, Thread};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Command to run
    #[clap(short, long, value_parser)]
    command: String,
    // count: u8,
    // count: u8,
    // count: u8,
    // count: u8,
    // count: u8,
    // count: u8,
    // count: u8,
}

#[derive(Serialize, Deserialize)]
struct WaybarOutput {
    text: String,
    class: String,
}

fn print_diff(diff: Duration) -> serde_json::Result<()> {
    let class = (if diff.num_minutes() > 60 { "warn" } else { "" }).to_owned();
    let text = format!("{:02}:{:02}", diff.num_hours(), diff.num_minutes() % 60).to_owned();
    let output = WaybarOutput { text, class };
    let j = serde_json::to_string(&output)?;
    println!("{}", j);
    Ok(())
}

fn write_stamp(path: &str) -> Result<(), Error> {
    let mut output = File::create(path)?;
    let now = Local::now().timestamp();
    write!(output, "{}\n", now)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let path = "timestamp.txt";
    let now = Local::now().timestamp();

    match args.command.as_str() {
        "diff" => {
            let f = File::open(path);

            match f {
                Ok(ff) => {
                    let buffered = BufReader::new(ff);
                    let s = buffered.lines().next().unwrap().unwrap();

                    let diff = NaiveDateTime::from_timestamp(now, 0)
                        - NaiveDateTime::from_timestamp(s.parse::<i64>().unwrap(), 0);
                    print_diff(diff)?;
                }

                Err(_) => {
                    write_stamp(path)?;
                }
            }
        }
        "stamp" => {
            write_stamp(path)?;
        }
        &_ => panic!("arg wrong"),
    }

    thread::sleep(time::Duration::from_millis(100));

    Ok(())
}
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
