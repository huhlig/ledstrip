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
use std::io::Write;
use std::net::TcpStream;

enum Command {
    On,
    Off,
    Color {
        r: u8,
        g: u8,
        b: u8,
    },
}

impl Command {
    fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = match *self {
            Command::On => vec![0x71, 0x23, 0x0f],
            Command::Off => vec![0x71, 0x24, 0x0f],
            Command::Color { r, g, b } => vec![0x31, r, g, b, 0x00, 0x00, 0x0f],
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
        .subcommand(SubCommand::with_name("color")
            .about("Set Strip Color")
            .arg(Arg::with_name("red")
                .help("Set Red Value")
                .short("r")
                .long("red")
                .required(true)
                .default_value("0")
            )
            .arg(Arg::with_name("green")
                .help("Set Green Value")
                .short("g")
                .long("green")
                .required(true)
                .default_value("0")
            )
            .arg(Arg::with_name("blue")
                .help("Set Blue Value")
                .short("b")
                .long("blue")
                .required(true)
                .default_value("0")
            )
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();

    match matches.subcommand() {
        ("on", Some(_)) => send_command(host, port, Command::On),
        ("off", Some(_)) => send_command(host, port, Command::Off),
        ("color", Some(matches)) => send_command(host, port, Command::Color {
            r: matches.value_of("red").unwrap().parse().expect("Unable to parse red value"),
            g: matches.value_of("green").unwrap().parse().expect("Unable to parse green value"),
            b: matches.value_of("blue").unwrap().parse().expect("Unable to parse blue value"),
        }),
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No subcommand was used.")),
    }.unwrap()
}

