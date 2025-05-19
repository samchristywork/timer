fn parse_time(input: String) -> u64 {
    let parts: Vec<&str> = input.split(':').collect();

    match parts.len() {
        1 => {
            let seconds = parts[0].parse::<u64>().expect("Invalid time format");
            seconds
        },
        2 => {
            let minutes = parts[0].parse::<u64>().expect("Invalid time format");
            let seconds = parts[1].parse::<u64>().expect("Invalid time format");
            minutes * 60 + seconds
        },
        3 => {
            let hours = parts[0].parse::<u64>().expect("Invalid time format");
            let minutes = parts[1].parse::<u64>().expect("Invalid time format");
            let seconds = parts[2].parse::<u64>().expect("Invalid time format");
            hours * 3600 + minutes * 60 + seconds
        },
        _ => panic!("Invalid time format"),
    }
}

fn main() {
    let command = std::env::args().nth(1).expect("No command provided");

    match command.as_str() {
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
