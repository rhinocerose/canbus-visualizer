mod configs;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use canparse::pgn::PgnLibrary;
use configs::Overview;
use std::fs;
use socketcan::CANSocket;

const DBC_FILE: &str        = "./data/anz.dbc";
const CAN_CHANNEL: &str     = "vcan0";
const CONFIG_FILE: &str     = "./data/config.yaml";

fn main() {

    let mut system_values   = Overview::new();
    let lib                 = PgnLibrary::from_dbc_file(DBC_FILE).expect("no such file");
    let mut socket_rx       = CANSocket::open(CAN_CHANNEL).expect("no such file");

    let file_parse: String = fs::read_to_string(CONFIG_FILE).expect("no such file");
    system_values.add_nodes(&file_parse);

    loop {

    }

}
