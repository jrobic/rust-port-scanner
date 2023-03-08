use std::{
    io::{self, Write},
    net::IpAddr,
    sync::mpsc::Sender,
};

use tokio::net::TcpStream;

pub async fn scan(tx: Sender<u16>, port: u16, host: IpAddr) {
    // Attempts Connects to the address on the given port.
    // If successful, print out a . and send the port to the channel
    // If not, do nothing.
    if (TcpStream::connect((host, port)).await).is_ok() {
        print!(".");
        io::stdout().flush().unwrap();
        tx.send(port).unwrap();
    }
}
