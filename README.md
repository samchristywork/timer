## Overview

This project provides a simple, cross-platform command-line timer tool written
in Rust.

It offers three modes of operation:

- Stopwatch
- Timer
- Alarm

The main goal of this project is to offer a versatile time-keeping utility that
can be readily integrated into scripts without relying on graphical interfaces
or external dependencies beyond the Rust ecosystem.

## Features

- Intuitive command-line interface
- Stopwatch mode counts time up until Ctrl+C
- Timer mode exits after time has elapsed
- Alarm mode exits at a specified time
- Alarm mode accepts both time and datetime inputs
- Continuously updating display of remaining/elapsed time
- Lightweight single binary with few dependencies
- Cross-platform and easy to build

## Setup

Just run:

```
cargo build --release
```

Then, just use the binary at `target/release/timer`.

## Usage

```
Usage: timer <command> [options]

Commands:
  stopwatch            Start a stopwatch
  timer <time>         Start a timer (format: HH:MM:SS)
  alarm <date> <time>  Set an alarm
  alarm <time>         Set an alarm

Options:
  -h, --help  Show this help message
```

## Examples

```
# Starts a stopwatch that will count up until Ctrl+C
timer stopwatch
```

```
# Sets a timer for ten minutes
timer timer 10:00
```

```
# Sets an alarm for 10PM
timer alarm 22:00:00
```

## Dependencies

```
Rust/Cargo
```

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
