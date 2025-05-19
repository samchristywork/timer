fn parse_time(input: String) -> u64 {
    let parts: Vec<&str> = input.split(':').collect();

    match parts.len() {
        1 => {
            let seconds = parts[0].parse::<u64>().expect("Invalid time format");
            seconds
        }
        2 => {
            let minutes = parts[0].parse::<u64>().expect("Invalid time format");
            let seconds = parts[1].parse::<u64>().expect("Invalid time format");
            minutes * 60 + seconds
        }
        3 => {
            let hours = parts[0].parse::<u64>().expect("Invalid time format");
            let minutes = parts[1].parse::<u64>().expect("Invalid time format");
            let seconds = parts[2].parse::<u64>().expect("Invalid time format");
            hours * 3600 + minutes * 60 + seconds
        }
        _ => panic!("Invalid time format"),
    }
}

fn flush() {
    use std::io::{self, Write};
    io::stdout().flush().expect("Failed to flush stdout");
}

fn print_time(seconds: u64) {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hours > 0 {
        print!("{}:{:02}:{:02}  ", hours, minutes, seconds);
    } else if minutes > 0 {
        print!("{}:{:02}  ", minutes, seconds);
    } else {
        print!("{}  ", seconds);
    }

    flush();
}

fn stopwatch() {
    println!("Press Ctrl+C to stop the stopwatch");

    let start = std::time::Instant::now();
    loop {
        let elapsed = start.elapsed().as_secs();
        print!("\rElapsed time: ");
        print_time(elapsed);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn hide_cursor() {
    use std::io::{self, Write};
    print!("\x1B[?25l");
    io::stdout().flush().expect("Failed to hide cursor");
}

fn usage() {
    println!("Usage: timer <command> [options]");
    println!();
    println!("Commands:");
    println!("  stopwatch     Start a stopwatch");
    println!();
    println!("Options:");
    println!("  -h, --help    Show this help message");
    std::process::exit(1);
}

fn main() {
    let command = std::env::args().nth(1).expect("No command provided");

    hide_cursor();
    ctrlc::set_handler(move || {
        println!("\nTimer stopped");
        print!("\x1B[?25h");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    match command.as_str() {
        "stopwatch" => {
            stopwatch();
        }
        "timer" => {
            if std::env::args().len() != 3 {
                usage();
                return;
            }

            let seconds = parse_time(std::env::args()
                .nth(2)
                .expect("No time provided"));
        },
        _ => usage(),
    }
}
