mod configs;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate lazy_static;

use anyhow::{Context, Result};
use canparse::pgn::{ParseMessage, PgnLibrary};
use configs::Overview;
use futures_util::stream::StreamExt;
use socketcan::CANFrame;
use tokio_socketcan::{CANSocket, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let lib_file =      "./data/anz.dbc";
    let canbus =        "vcan0";
    let config_file =   "./data/config.yaml";


    let mut system_values = Overview::new();
    let lib = PgnLibrary::from_dbc_file(lib_file)?;
    let mut socket_rx = CANSocket::open(canbus)?;

    let file_parse: String = std::fs::read_to_string(config_file).expect("no such file");
    system_values.add_nodes(&file_parse);

    while let Some(Ok(frame)) = socket_rx.next().await {
        parse_frame(&lib, &frame, &mut system_values).expect("Unable to read frame");
    }
    Ok(())
}

fn read_frame(
    target: &str,
    library: &PgnLibrary,
    frame: &CANFrame,
) -> Result<f32, Box<dyn std::error::Error>> {
    // println!("{:?}", target);
    let parsed = library
        .get_spn(target)
        .unwrap()
        .parse_message(frame.data())
        .with_context(|| format!("could not find target string `{}`", target))?;
    Ok(parsed)
}

fn parse_frame(
    library: &PgnLibrary,
    frame: &CANFrame,
    values: &mut Overview,
) -> Result<(), Box<dyn std::error::Error>> {
    for iterator in values.match_frame(frame.id()) {
        values.update_entry(iterator, read_frame(iterator, &library, frame).unwrap());
    }
    values.process();
    Ok(())
}
