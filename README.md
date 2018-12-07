# LED Strip Controller

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.org/huhlig/ledstrip.svg?branch=master)](https://travis-ci.org/huhlig/ledstrip)

Controller application for Sanwo WiFi LED Strip Light Kit.
Available from Amazon https://smile.amazon.com/gp/product/B072Q4SMX6
 
```
ledstrip 0.1.0
Hans W. Uhlig <huhlig@gmail.com>
Control Sanwo WiFi LED Strip Light Kit

USAGE:
    ledstrip --host <host> --port <port> [SUBCOMMAND]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>    Set LED Strip Hostname
    -p, --port <port>    Set LED Strip Port [default: 5577]

SUBCOMMANDS:
    color    Set Strip Color
    help     Prints this message or the help of the given subcommand(s)
    off      Turn Strip Off
    on       Turn Strip On
```