mod configs;

use canparse::pgn::{PgnLibrary, ParseMessage};
use futures_util::stream::{StreamExt};
use tokio_socketcan::{CANSocket, Error};
use socketcan::{CANFrame};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let lib_file = "./data/anz.dbc";
    let canbus = "vcan0";

    let mut system_values = configs::Overview::new();
    let lib = PgnLibrary::from_dbc_file(lib_file).unwrap();
    let mut socket_rx = CANSocket::open(canbus)?;

    let test_struct = configs::NodeValue::new("temperature_diode", "Diode Temperature".to_string(), 406768872);
    system_values.join(test_struct.clone());
    let test_struct = configs::NodeValue::new("temperature_contactor", "Contactor Temperature".to_string(), 406768872);
    system_values.join(test_struct.clone());
    let test_struct = configs::NodeValue::new("temperature_min_mono", "Min Temperature".to_string(), 406768872);
    system_values.join(test_struct.clone());
    let test_struct = configs::NodeValue::new("temperature_max_mono", "Max Temperature".to_string(), 406768872);
    system_values.join(test_struct.clone());
    let test_struct = configs::NodeValue::new("voltage_min_mono", "Min Voltage".to_string(), 406768872);
    system_values.join(test_struct.clone());
    let test_struct = configs::NodeValue::new("voltage_max_mono", "Max Voltage".to_string(), 406768872);
    system_values.join(test_struct.clone());

    while let Some(Ok(frame)) = socket_rx.next().await {
        parse_frame(&lib, &frame, &mut system_values).expect("Unable to read frame");
        for (key, value) in &system_values.hash_map {
            println!("{} {:?}", key, value);
        }
        system_values.increment();
    }
    Ok(())
}

fn read_frame(target: &str, library: &PgnLibrary, frame: &CANFrame) -> Result<f32, Error> {
    let parsed = library.get_spn(target).unwrap().parse_message(frame.data()).expect("Unknown frame");
    Ok(parsed)
}

fn parse_frame(library: &PgnLibrary, frame: &CANFrame, values: &mut configs::Overview) -> Result<(), Error> {
    for iterator in values.match_frame(frame.id()) {
        values.update_entry(iterator, read_frame(iterator, &library, frame).unwrap());
    }
    Ok(())
}
