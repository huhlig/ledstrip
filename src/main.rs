//
// Copyright 2017 Hans W. Uhlig.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use clap::{App, Arg, SubCommand};
use rand::{thread_rng, Rng};
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
    pub fn red() -> Color {
        Color { r: 255, g: 0, b: 0 }
    }
    pub fn green() -> Color {
        Color { r: 0, g: 255, b: 0 }
    }
    pub fn blue() -> Color {
        Color { r: 0, g: 0, b: 255 }
    }
    pub fn yellow() -> Color {
        Color { r: 255, g: 255, b: 0 }
    }
    pub fn cyan() -> Color {
        Color { r: 0, g: 255, b: 255 }
    }
    pub fn magenta() -> Color {
        Color { r: 255, g: 0, b: 255 }
    }
    pub fn white() -> Color {
        Color { r: 255, g: 255, b: 255 }
    }
}

enum Command {
    On,
    Off,
    Color(Color),
}

impl Command {
    fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = match self {
            Command::On => vec![0x71, 0x23, 0x0f],
            Command::Off => vec![0x71, 0x24, 0x0f],
            Command::Color(color) => vec![0x31, color.r, color.g, color.b, 0x00, 0x00, 0x0f],
        };
        let checksum: u8 = buffer.iter().sum::<u8>() & 0xFF;
        buffer.push(checksum);
        return buffer;
    }
}

fn send_command(host: &str, port: &str, command: Command) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let buffer = command.to_buffer();
    stream.write_all(&buffer)?;
    stream.flush()?;
    Ok(())
}

fn main() {
    let matches = App::new("ledstrip")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Hans W. Uhlig <huhlig@gmail.com>")
        .about("Control Sanwo WiFi LED Strip Light Kit")
        .arg(Arg::with_name("host")
            .help("Set LED Strip Hostname")
            .short("h")
            .long("host")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("port")
            .help("Set LED Strip Port")
            .short("p")
            .long("port")
            .required(true)
            .default_value("5577")
        )
        .subcommand(SubCommand::with_name("on")
            .about("Turn Strip On")
        )
        .subcommand(SubCommand::with_name("off")
            .about("Turn Strip Off")
        )
        .subcommand(SubCommand::with_name("rave")
            .about("LED Rave Mode")
            .arg(Arg::with_name("time")
                .help("Set Rave Time Delay")
                .short("t")
                .long("time")
                .required(true)
                .default_value("200")
            )
        )
        .subcommand(SubCommand::with_name("color")
            .about("Set Strip Color")
            .arg(Arg::with_name("name")
                .help("Set Color By Name")
                .short("n")
                .long("name")
                .takes_value(true)
                .required(false)
                .conflicts_with("red")
                .conflicts_with("green")
                .conflicts_with("blue")
            )
            .arg(Arg::with_name("red")
                .help("Set Red Value")
                .short("r")
                .long("red")
                .required_unless("name")
                .default_value("0")
            )
            .arg(Arg::with_name("green")
                .help("Set Green Value")
                .short("g")
                .long("green")
                .required_unless("name")
                .default_value("0")
            )
            .arg(Arg::with_name("blue")
                .help("Set Blue Value")
                .short("b")
                .long("blue")
                .required_unless("name")
                .default_value("0")
            )
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();

    match matches.subcommand() {
        ("on", Some(_)) => send_command(host, port, Command::On),
        ("off", Some(_)) => send_command(host, port, Command::Off),
        ("rave", Some(matches)) => {
            let time = matches.value_of("time").unwrap()
                .parse().expect("Unable to parse time value");
            let mut rng = thread_rng();
            loop {
                send_command(host, port, Command::Color(Color::new(
                    rng.gen_range(0, 255),
                    rng.gen_range(0, 255),
                    rng.gen_range(0, 255),
                ))).unwrap();
                sleep(Duration::from_millis(time));
            }
        }
        ("color", Some(matches)) => send_command(host, port, Command::Color(
            if matches.is_present("name") {
                match matches.value_of("name") {
                    Some("black") => Color::black(),
                    Some("red") => Color::red(),
                    Some("green") => Color::green(),
                    Some("blue") => Color::blue(),
                    Some("yellow") => Color::yellow(),
                    Some("cyan") => Color::cyan(),
                    Some("magenta") => Color::magenta(),
                    Some("white") => Color::white(),
                    Some(color) => panic!("Unknown Color {}", color),
                    None => panic!("Color not specified"),
                }
            } else {
                Color::new(
                    matches.value_of("red").unwrap()
                        .parse().expect("Unable to parse red value"),
                    matches.value_of("green").unwrap()
                        .parse().expect("Unable to parse green value"),
                    matches.value_of("blue").unwrap()
                        .parse().expect("Unable to parse blue value"),
                )
            }
        )),
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No subcommand was used.")),
    }.unwrap()
}

