use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Command to run
    #[clap(short, long, value_parser)]
    command: String,
    // #[clap(short, long, value_parser, default_value_t = 1)]
    // count: u8,
}

// fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
//     date_time.checked_sub_signed(Duration::days(1))
// }

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if args.command == "stamp" {
        // let now = Utc::now();

        // let (is_pm, hour) = now.hour12();
        // println!(
        //     "The current UTC time is {:02}:{:02}:{:02} {}",
        //     hour,
        //     now.minute(),
        //     now.second(),
        //     if is_pm { "PM" } else { "AM" }
        // );

        // let almost_three_weeks_from_now = now
        //     .checked_add_signed(Duration::weeks(2))
        //     .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        //     .and_then(day_earlier);

        let local_time = Local::now();
        // println!("Local time now is {}", local_time);

        let a = local_time.to_rfc3339();
        println!("Local time now is {}", local_time.timestamp());
        println!("Local time now is {}", local_time.to_rfc2822());
        let date_only = NaiveDateTime::parse_from_str("1661380553", "%s").unwrap();
        println!("{}", date_only);
        let s = local_time.timestamp();
        println!("{}", s);
        // println!("stamp {}", NaiveDateTime::parse_from_str(&s, "%s").unwrap());
        // writeln!()

        // match almost_three_weeks_from_now {
        //     Some(x) => println!("{}", x),
        //     None => eprintln!("Almost three weeks from now overflows!"),
        // }
        let path = "timestamp.txt";

        let exists = Path::new(path).exists();
        println!("{}", exists);

        let mut output = File::create(path)?;
        write!(output, "{}\n", local_time)?;

        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        // for line in buffered.lines() {
        //     let stamp = line.unwrap();
        //     let time_read = DateTime::parse_from_rfc2822(&stamp).unwrap();
        //

        let ts = buffered.lines().next().unwrap().unwrap();
        println!("{}", ts);

        let k = NaiveDateTime::from_timestamp(s, 0) - date_only;
        println!("diff is {}", k.num_minutes());
        println!(
            "diff is {:02}:{:02}:{:02}",
            k.num_hours(),
            k.num_minutes() % 60,
            k.num_seconds() % 60
        );

        println!("diff is {:?}", k);
        //     println!("{}", line?);
        // }
    }
    Ok(())
    // for _ in 0..args.command {
    //     println!("Hello {}!", args.name)
    // }
}
