use clap::Parser;
use clap_num::maybe_hex;
use regex::Regex;
use serialport::{SerialPortType, available_ports};

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Filter ports that match the specified USB product name
    #[arg(short, long)]
    name: Option<String>,
    /// Filter ports that match the specified USB PID
    #[arg(short, long, value_parser=maybe_hex::<u16>)]
    pid: Option<u16>,
    /// Filter ports that match the specified USB VID
    #[arg(short, long, value_parser=maybe_hex::<u16>)]
    vid: Option<u16>,
    /// Prints only the port names
    #[arg(short, long, default_value_t = false)]
    only_port_name: bool,
}

fn main() {
    // initialise logging
    pretty_env_logger::init();
    info!("lstty - list serial ports");

    let cli = Cli::parse();
    let cli_name_regex: Option<Regex> = cli.name.map(|name| Regex::new(&name).unwrap());

    // print serial ports
    match available_ports() {
        Ok(ports) => {
            info!("{} serial ports found:", ports.len());

            let ports = ports.iter().filter(|p| {
                cli_name_regex.as_ref().is_none_or(|cli_name_regex| {
                    // Filter by product name
                    matches!(
                        &p.port_type,
                        SerialPortType::UsbPort(info) if info.product
                            .as_ref()
                            .is_some_and(|product_name| cli_name_regex.is_match(product_name))
                    )
                }) && cli.pid.as_ref().is_none_or(|cli_pid| {
                    // Filter by PID
                    matches!(
                        &p.port_type,
                        SerialPortType::UsbPort(info) if info.pid == *cli_pid
                    )
                }) && cli.vid.as_ref().is_none_or(|cli_vid| {
                    // Filter by VID
                    matches!(
                        &p.port_type,
                        SerialPortType::UsbPort(info) if info.vid == *cli_vid
                    )
                })
            });

            for port in ports {
                // determine port details string
                let mut details = String::new();

                // add port type
                details.push_str(
                    format!(
                        "{:9}",
                        match port.port_type {
                            SerialPortType::BluetoothPort => "bluetooth",
                            SerialPortType::PciPort => "pci",
                            SerialPortType::UsbPort(_) => "usb",
                            SerialPortType::Unknown => "unknown",
                        }
                    )
                    .as_str(),
                );

                // if the port is a usb device, add extra info
                if let SerialPortType::UsbPort(info) = &port.port_type {
                    details.push_str(
                        format!(
                            "{:04x}:{:04x} {}",
                            info.vid,
                            info.pid,
                            info.product.as_ref().unwrap_or(&String::new())
                        )
                        .as_str(),
                    );
                }

                // print port details
                if cli.only_port_name {
                    println!("{}", port.port_name);
                } else {
                    println!("{:14} {}", port.port_name, details);
                }
            }
        }

        Err(e) => {
            error!("Failed to retrieve serial ports: {e}");
        }
    }
}
