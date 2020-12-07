#[allow(unused)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use clap::{App, Arg};

macro_rules! day {
    ($fmt: expr, $($day: ident),*) => {
        match ($fmt as &str) {
            $(
                stringify!($day) => $day::main(),
            )*
            _ =>  eprintln!("Error: Day not found (yet?)."),
        }
    }
}

fn main() {
    let matches = App::new("Advent of Code")
        .arg(
            Arg::with_name("day")
            .short("d")
            .long("day")
            .help("The day number to execute")
            .takes_value(true)
        ).get_matches();

        

    match matches.value_of("day") {
        None => eprintln!("Error: Argument 'day' not found."),
        Some(i) => match i.parse::<i64>() {
            Err(_) => eprintln!("Error: Couldn't parse argument 'day' as integer."),
            Ok(i) => day!(&format!("day{}", i),
                day1, 
                day2,
                day3,
                day4,
                day5,
                day6,
                day7
            )
        }
    }
}
