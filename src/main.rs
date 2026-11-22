use cfonts::{Colors, Fonts, Options, Rgb, render};
use clap::Parser;
use owo_colors::OwoColorize;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Animated FRC Pit Clock")]
struct Args {
    minutes: f64,

    #[arg(short, long, default_value = "MATCH TIME!")]
    message: String,
}

fn main() {
    let args = Args::parse();
    let total_duration = Duration::from_secs_f64(args.minutes * 60.0);
    let start = Instant::now();

    // Prevent scrolling/stacking
    print!("\x1B[?1049h\x1B[?25l");
    io::stdout().flush().unwrap();

    loop {
        let elapsed = start.elapsed();
        if elapsed >= total_duration {
            break;
        }

        let remaining = total_duration - elapsed;
        let rem_secs = remaining.as_secs();
        let mins = rem_secs / 60;
        let secs = rem_secs % 60;
        let progress = remaining.as_secs_f64() / total_duration.as_secs_f64();

        // Update Catppuccin Mocha colors
        let (r, g, b) = if progress > 0.5 {
            (166, 227, 161) // Catppuccin Mocha Green
        } else if progress > 0.2 {
            (249, 226, 175) // Catppuccin Mocha Yellow
        } else {
            (243, 139, 168) // Catppuccin Mocha Red
        };

        // Block digits
        let time_str = format!("{:02}:{:02}", mins, secs);
        let big_clock = render(Options {
            text: time_str,
            font: Fonts::FontBlock,
            colors: vec![Colors::Rgb(Rgb::Val(r, g, b))],
            spaceless: true,
            ..Options::default()
        });

        // Progress bar
        let bar_width = 30;
        let filled = ((progress * bar_width as f64).round() as usize).min(bar_width);
        let empty = bar_width - filled;
        let bar = format!(
            "[{}{}] {:>3}%",
            "█".repeat(filled).truecolor(r, g, b),
            "░".repeat(empty).truecolor(88, 91, 112),
            (progress * 100.0).round() as u8
        );

        // Spinner animation
        let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let spin_idx = (elapsed.as_millis() / 100) as usize % spinners.len();
        let header = format!(
            "{}  {}  {}",
            spinners[spin_idx].truecolor(203, 166, 247), // Mauve
            args.message.bold().truecolor(205, 214, 244),
            spinners[spin_idx].truecolor(203, 166, 247)
        );

        // Clean clock
        let clean_clock = big_clock
            .text
            .lines()
            .map(|line| format!("{}\x1B[K", line))
            .collect::<Vec<_>>()
            .join("\n");

        print!(
            "\x1B[H{}\x1B[K\n{}\n  {}\x1B[K\x1B[J",
            header, clean_clock, bar
        );

        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    // Flashing animation
    for i in 0..6 {
        let color = if i % 2 == 0 {
            Rgb::Val(203, 166, 247) // Catppuccin Mauve
        } else {
            Rgb::Val(243, 139, 168) // Catppuccin Red
        };

        let banner = render(Options {
            text: args.message.clone(),
            font: Fonts::FontBlock,
            colors: vec![Colors::Rgb(color)],
            ..Options::default()
        });

        print!("\x1B[H{}\x1B[J\x07", banner.text);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(350));
    }

    // Restore normal terminal screen & cursor
    print!("\x1B[?25h\x1B[?1049l");
    println!("{}", args.message);
}
