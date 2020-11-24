mod configs;

use configs::{Overview};
use canparse::pgn::{PgnLibrary, ParseMessage};
use futures_util::stream::{StreamExt};
use tokio_socketcan::{CANSocket, Error};
use socketcan::{CANFrame};
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let lib_file = "./data/anz.dbc";
    let canbus = "vcan0";

    let mut system_values = Overview::new();
    let lib = PgnLibrary::from_dbc_file(lib_file)?;
    let mut socket_rx = CANSocket::open(canbus)?;

    system_values.add_node("temperature_diode", "Diode Temperature", 406768872);
    system_values.add_node("temperature_contactor", "Contactor Temperature", 406768872);
    system_values.add_node("temperature_min_mono", "Min Temperature", 406768872);
    system_values.add_node("temperature_max_mono", "Max Temperature", 406768872);
    system_values.add_node("voltage_min_mono", "Min Voltage", 406768872);
    system_values.add_node("voltage_max_mono", "Max Voltage", 406768872);

    while let Some(Ok(frame)) = socket_rx.next().await {
        parse_frame(&lib, &frame, &mut system_values).expect("Unable to read frame");
        system_values.print_info();
        system_values.increment();
    }
    Ok(())
}

fn read_frame(target: &str, library: &PgnLibrary, frame: &CANFrame) -> Result<f32, Box<dyn std::error::Error>> {
    let parsed = library.get_spn(target).unwrap()
        .parse_message(frame.data())
        .with_context(|| format!("could not find target string `{}`", target))?;
    Ok(parsed)
}

fn parse_frame(library: &PgnLibrary, frame: &CANFrame, values: &mut Overview) -> Result<(), Box<dyn std::error::Error>> {
    for iterator in values.match_frame(frame.id()) {
        values.update_entry(iterator, read_frame(iterator, &library, frame).unwrap());
    }
    Ok(())
}
