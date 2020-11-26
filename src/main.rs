mod configs;

#[macro_use]
extern crate serde_derive;
extern crate toml;

use anyhow::{Context, Result};
use canparse::pgn::{ParseMessage, PgnLibrary};
use configs::Overview;
use futures_util::stream::StreamExt;
use socketcan::CANFrame;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use tokio_socketcan::{CANSocket, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let lib_file = "./data/anz.dbc";
    let canbus = "vcan0";
    let config_file = "./data/config.yaml";

    let mut system_values = Overview::new();
    let lib = PgnLibrary::from_dbc_file(lib_file)?;
    let mut socket_rx = CANSocket::open(canbus)?;

    create_struct(config_file);

    system_values.add_node("temperature_diode", "Diode Temperature", 406768872);
    system_values.add_node("temperature_contactor", "Contactor Temperature", 406768872);
    system_values.add_node("temperature_min_mono", "Min Temperature", 406768872);
    system_values.add_node("temperature_max_mono", "Max Temperature", 406768872);
    system_values.add_node("voltage_min_mono", "Min Voltage", 406768872);
    system_values.add_node("voltage_max_mono", "Max Voltage", 406768872);

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
    values.match_state();
    values.print_info();
    values.increment();
    Ok(())
}

// fn (filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("no such file");
//     let buf = BufReader::new(file);
//     buf.lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect()
// }

fn create_struct(filename: impl AsRef<Path>) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let buf: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    for mut line in buf {
        line.retain(|c| c != '"');
        // let space_offset = line.find(':').unwrap_or(line.len());
        line.drain(..line.find(':').unwrap_or(line.len()));
        // line.remove(0);
        println!("{:?}", line);
    }
}
