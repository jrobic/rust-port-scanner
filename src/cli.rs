use std::net::{IpAddr, Ipv4Addr};

use clap::{value_parser, Arg, Command};

pub struct Argument {
    pub host: IpAddr,
    pub start_port: u16,
    pub end_port: u16,
}

const IP_FALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

pub fn get_cli_args() -> Argument {
    let matches = Command::new("Port Scanner")
        .version("1.0.0")
        .author("Jonathan Robic<hello@jonathanrobic.fr>")
        .about("Scan all ports for specific host")
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_name("HOST")
                .help("Host to scan"),
        )
        .arg(
            Arg::new("start_port")
                .short('S')
                .long("start_port")
                .value_name("START_PORT")
                .help("Start port to scan")
                .value_parser(value_parser!(u16))
                .default_value("1"),
        )
        .arg(
            Arg::new("end_port")
                .short('E')
                .long("end_port")
                .value_name("END_PORT")
                .help("End port to scan")
                .value_parser(value_parser!(u16))
                .default_value("65535"),
        )
        .get_matches();

    let host = match matches.get_one::<String>("host") {
        Some(host) => host.parse::<IpAddr>().unwrap_or(IP_FALLBACK),
        None => IP_FALLBACK,
    };

    let start_port = matches
        .get_one::<u16>("start_port")
        .expect("Start port is required");
    let end_port = matches
        .get_one::<u16>("end_port")
        .expect("End port is required");

    Argument {
        host,
        start_port: start_port.to_owned(),
        end_port: end_port.to_owned(),
    }
}
