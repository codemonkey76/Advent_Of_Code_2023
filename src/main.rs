pub mod day01;

use std::time::{Duration, Instant};
use clap::Parser;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result as FmtResult, Result as DisplayResult};
use num_format::{Locale, ToFormattedString};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();

    let day = args.day;
    let all = args.all;

    let timer = Instant::now();

    if all {
        let mut results = Vec::with_capacity(25);
        println!("Running all days");
        (1..=25).for_each(|day| results.push(run_day(day, &timer)));
        results.iter().for_each(|result| println!("{}", result));
        return;
    }

    if let Some(day) = day {
        println!("Day: {:?}", day);
        println!("{}", run_day(day, &timer));
        return;
    }

    println!("No day specified");
}

fn run_day(day: u8, timer: &Instant) -> RunResult {
    println!("Running day: {}", day);

    let run_both = match day {
        1 => day01::run_both,
        _ => panic!("Day not implemented"),
    };

    let start = timer.elapsed();
    let (answer_one, answer_two) = run_both();
    let duration = timer.elapsed() - start;

    RunResult {
        day,
        answer_one,
        answer_two,
        duration
    }
}

struct RunResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
    duration: Duration
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let microseconds = self.duration.as_secs_f32()*1_000_000.0;
        writeln!(f, "***************************************************")?;
        writeln!(f, "*    Advent of Code: 2023, Day {:0>2}                 *", self.day)?;
        writeln!(f, "*        Solution for...                          *")?;
        writeln!(f, "*            Part One: {:>12}               *", self.answer_one.to_string())?;
        writeln!(f, "*            Part Two: {:>12}               *", self.answer_two.to_string())?;
        writeln!(f, "*    Run Time: {:>10.4}µs                       *", microseconds)?;
        writeln!(f, "***************************************************")
    }
}

macro_rules! impl_output_from {
     ($(($e:tt, $t:ty) ),*) => {
        #[derive(Debug, Eq)]
        pub enum Output {
            $( $e($t), )*
        }

        $(
            impl From<$t> for Output {
                fn from(value: $t) -> Self { Output::$e(value) }
            }
        )*

        impl Display for Output {
            fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
                match self {
                    $(
                    Output::$e(v) => write!(f, "{}", v.to_formatted_string(&Locale::en)),
                    )*
                }
            }
        }
    }
}

impl_output_from! {
    (U8, u8),
    (U16, u16),
    (U32, u32),
    (U64, u64),
    (U128, u128),
    (I8, i8),
    (I16, i16),
    (I32, i32),
    (I64, i64),
    (I128, i128)
}

impl<T: Display> PartialEq<T> for Output {
    fn eq(&self, other: &T) -> bool {
        *self.to_string() == other.to_string()
    }
}

pub enum Part {
    One,
    Two,
}


