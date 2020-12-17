mod configs;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use canparse::pgn::PgnLibrary;
use configs::Overview;
use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};
use std::fs;

const DBC_FILE: &str        = "./data/anz.dbc";
const CAN_CHANNEL: &str     = "vcan0";
const CONFIG_FILE: &str     = "./data/config.yaml";

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut system_values   = Overview::new();
    let lib                 = PgnLibrary::from_dbc_file(DBC_FILE)?;
    let mut socket_rx       = CANSocket::open(CAN_CHANNEL)?;

    let file_parse: String = fs::read_to_string(CONFIG_FILE).expect("Invalid config file");
    system_values.add_nodes(&file_parse);

    while let Some(Ok(frame)) = socket_rx.next().await {
        system_values.parse_frame(&lib, &frame).expect("Unable to read frame");
    }
    Ok(())
}
