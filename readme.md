# lstty

<a href="https://crates.io/crates/lstty"><img src=https://img.shields.io/crates/v/lstty></img></a>

This is a simple terminal tool to list what serial ports there are.  It prints the name of the serial port, what type of port it is, and if it's a USB port it will print the VID and PID and product name.  At the minute that's all it does.  There aren't any command line parameters.  It's just `lstty`.

An example of this program's output:
```text
$ lstty
/dev/ttyACM0   usb      16c0:048a minimixer
/dev/ttyACM1   usb      303a:1001 USB_JTAG_serial_debug_unit
/dev/ttyACM2   usb      04d8:00dd MCP2221(a) UART/I2C Bridge
/dev/ttyS0     unknown 
```

## Filtering

This program allows to filter USB serial devices by product-name like:

```bash
$ lstty -n "STM32 Virtual ComPort"
/dev/ttyACM0   usb      0483:5740 STM32 Virtual ComPort
```

by product ID (PID)

```bash
$ lstty -n --pid 0x5740
/dev/ttyACM0   usb      0483:5740 STM32 Virtual ComPort
```

or by vendor ID (VID)

```bash
$ lstty -n --vid 0x0483
/dev/ttyACM0   usb      0483:5740 STM32 Virtual ComPort
```

or by any combination of those.

You can also use the option `-o` or `--only-port-name` to print only the device serial port's name.

```bash
$ lstty -n "STM32 Virtual ComPort" --pid 0x5740 --vid 0x0483 -o
/dev/ttyACM0
```

## Debug

This program has a little bit of logging that can be enabled using the `RUST_LOG=<level>` environment variable.  `<level>` can be one of `trace`, `debug`, `info`, `warn`, or `error`.

## Installing

The tool can be installed from [crates.io](https://crates.io/crates/lstty) by running `cargo install lstty`.

## Building
This project is made with Rust, so make sure cargo is installed.  To build, just run `cargo build`, and to run use `cargo run`.

To install from source use `cargo install --path .`.

## Dependancies
This tool gets serial port info from the [serialport](https://github.com/serialport/serialport-rs) crate.  Logging is done using the [pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger) crate.
As command line tool library, it uses [clap](https://docs.rs/clap/) and the [clap-num](https://docs.rs/clap-num/).