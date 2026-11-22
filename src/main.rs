use clap::Parser;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    minutes: u64,

    #[arg(short, long, default_value = "Time's up!")]
    message: String,
}

fn main() {
    let args = Args::parse();
    let total_seconds = args.minutes * 60;
    for remaining_seconds in (0..=total_seconds).rev() {
        let minutes = remaining_seconds / 60;
        let seconds = remaining_seconds % 60;
        print!("\r{:02}:{:02}", minutes, seconds);
        Write::flush(&mut stdout()).unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\n{}", args.message);
}
