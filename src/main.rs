use chrono::prelude::*;
use std::env::args;
use std::io::Write;
use std::io::stdout;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn parse_time(input: String) -> u64 {
    let parts: Vec<&str> = input.split(':').collect();

    match parts.len() {
        1 => parts[0].parse::<u64>().expect("Invalid time format"),
        2 => {
            60 * parts[0].parse::<u64>().expect("Invalid time format")
                + parts[1].parse::<u64>().expect("Invalid time format")
        }
        3 => {
            3600 * parts[0].parse::<u64>().expect("Invalid time format")
                + 60 * parts[1].parse::<u64>().expect("Invalid time format")
                + parts[2].parse::<u64>().expect("Invalid time format")
        }
        _ => panic!("Invalid time format"),
    }
}

fn print_time(seconds: u64) {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;

    if h > 0 {
        print!("{}:{:02}:{:02}  ", h, m, s);
    } else if m > 0 {
        print!("{}:{:02}  ", m, s);
    } else {
        print!("{}  ", s);
    }

    stdout().flush().expect("Failed to flush stdout");
}

fn stopwatch() {
    println!("Press Ctrl+C to stop the stopwatch");

    let start = Instant::now();
    loop {
        print!("\rElapsed time: ");
        print_time(start.elapsed().as_secs());
        sleep(Duration::from_secs(1));
    }
}

fn timer(seconds: u64) {
    println!("Press Ctrl+C to stop the timer");

    let start = Instant::now();
    loop {
        let remaining = seconds - start.elapsed().as_secs();
        if remaining <= 0 {
            println!("\nTime's up!");
            break;
        }
        print!("\rTime remaining: ");
        print_time(remaining);
        sleep(Duration::from_secs(1));
    }
}

fn alarm_date_time(date: String, time: String) {
    let alarm_time = Local
        .datetime_from_str(&format!("{} {}", date, time), "%Y-%m-%d %H:%M:%S")
        .expect("Invalid date/time format");

    if alarm_time < Local::now() {
        println!("Alarm time is in the past");
        return;
    }

    println!("Alarm set for {}", alarm_time);
    loop {
        let remaining = alarm_time - Local::now();
        if remaining.num_seconds() <= 0 {
            println!("\nTime's up!");
            break;
        }
        print!("\rTime remaining: ");
        print_time(remaining.num_seconds() as u64);
        sleep(Duration::from_secs(1));
    }
}

fn alarm_time(time: String) {
    let date = Local::now().format("%Y-%m-%d").to_string();

    let alarm_time = Local
        .datetime_from_str(&format!("{} {}", date, time), "%Y-%m-%d %H:%M:%S")
        .expect("Invalid time format");

    let alarm_time = if alarm_time < Local::now() {
        alarm_time + chrono::Duration::days(1)
    } else {
        alarm_time
    };

    println!("Alarm set for {}", alarm_time);
    loop {
        let remaining = alarm_time - Local::now();
        if remaining.num_seconds() <= 0 {
            println!("\nTime's up!");
            break;
        }
        print!("\rTime remaining: ");
        print_time(remaining.num_seconds() as u64);
        sleep(Duration::from_secs(1));
    }
}

fn usage() {
    println!("Usage: timer <command> [options]");
    println!();
    println!("Commands:");
    println!("  stopwatch            Start a stopwatch");
    println!("  timer <time>         Start a timer (format: HH:MM:SS)");
    println!("  alarm <date> <time>  Set an alarm");
    println!("  alarm <time>         Set an alarm");
    println!();
    println!("Options:");
    println!("  -h, --help  Show this help message");
}

fn main() {
    let command = std::env::args().nth(1).expect("No command provided");

    print!("\x1B[?25l");
    ctrlc::set_handler(move || {
        println!("\nTimer stopped");
        print!("\x1B[?25h");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    match args().nth(1).expect("No command provided").as_str() {
        "stopwatch" => {
            stopwatch();
        }
        "timer" => {
            if args().len() != 3 {
                usage();
                return;
            }

            let seconds = parse_time(std::env::args().nth(2).expect("No time provided"));
            timer(seconds);
        }
        _ => usage(),
    }
}
