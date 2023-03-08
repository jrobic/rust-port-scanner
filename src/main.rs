use std::sync::mpsc::{channel, Receiver, Sender};

use tokio::task;

mod cli;
mod scan;

#[tokio::main]
async fn main() {
    // collect the arguments
    let argument = cli::get_cli_args();

    // initialize the channel
    let (tx, rx): (Sender<u16>, Receiver<u16>) = channel();

    println!(
        "Scanning ports {}-{} for {}",
        argument.start_port, argument.end_port, argument.host
    );

    // Iterate through all ports so that we can spwan single task for each
    for i in argument.start_port..argument.end_port {
        let tx = tx.clone();

        // Spawn a new task for each port
        task::spawn(async move { scan::scan(tx, i, argument.host).await });
    }

    // create a vector for all the outputs
    let mut outputs = vec![];

    // drop the tx clones
    drop(tx);

    // wait for all outputs to finish and push them into the vector
    for p in rx {
        outputs.push(p);
    }
    outputs.sort();

    println!();

    for v in outputs {
        println!("Port {} is used", v);
    }
}
