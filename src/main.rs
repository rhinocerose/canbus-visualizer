mod configs;

use std::collections::HashMap;
use canparse::pgn::{PgnLibrary, ParseMessage};
use futures_util::stream::{StreamExt};
use tokio_socketcan::{CANSocket, Error};
use socketcan::{CANFrame};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let mut sys_val = configs::SystemValues::new();
    // let mut dbc_numbers: HashMap<&str, f32> = HashMap::new();
    let mut test_map = configs::Overview::new();
    let lib = PgnLibrary::from_dbc_file("./data/anz.dbc").unwrap();
    let mut socket_rx = CANSocket::open("vcan0")?;

    let test_struct = configs::CollectedValues::new("temperature_diode", "Diode Temperature".to_string());
    test_map.join(test_struct.clone());
    let test_struct = configs::CollectedValues::new("temperature_contactor", "Contactor Temperature".to_string());
    test_map.join(test_struct.clone());
    let test_struct = configs::CollectedValues::new("temperature_min_mono", "Min Temperature".to_string());
    test_map.join(test_struct.clone());
    let test_struct = configs::CollectedValues::new("temperature_max_mono", "Max Temperature".to_string());
    test_map.join(test_struct.clone());
    let test_struct = configs::CollectedValues::new("voltage_min_mono", "Min Voltage".to_string());
    test_map.join(test_struct.clone());
    let test_struct = configs::CollectedValues::new("voltage_max_mono", "Max Voltage".to_string());
    test_map.join(test_struct.clone());

    while let Some(Ok(frame)) = socket_rx.next().await {
        // parse_frame(&lib, &frame, &mut sys_val, &mut dbc_numbers);
        parse_frame(&lib, &frame, &mut test_map);
        // for (key, value) in &dbc_numbers {
        //     println!("{} {}", key, value);
        // }
        for (key, value) in &test_map.hash_map {
            println!("{} {:?}", key, value);
        }
        // socket_tx.write_frame(frame)?.await;
    }
    Ok(())
}

fn read_frame(target: &str, library: &PgnLibrary, frame: &CANFrame) -> Result<f32, Error> {
    let parsed = library.get_spn(target).unwrap().parse_message(frame.data()).unwrap();
    Ok(parsed)
}

fn parse_frame(library: &PgnLibrary, frame: &CANFrame, values: &mut configs::Overview) {
    //println!("{:?}", frame);
    if frame.id() == 0x183ECCE8 {
        for iterator in &["temperature_diode", "temperature_contactor", "temperature_min_mono",
                          "temperature_max_mono", "voltage_max_mono", "voltage_min_mono"] {
            values.update_entry(iterator, read_frame(iterator, &library, frame).unwrap());
        }
    }
}

fn parse_frame2(library: &PgnLibrary, frame: &CANFrame, map: &mut HashMap<&str, f32>) {
    //println!("{:?}", frame);
    if frame.id() == 0x183ECCE8 {
        for iterator in &["temperature_diode", "temperature_contactor", "temperature_min_mono",
                          "temperature_max_mono", "voltage_max_mono", "voltage_min_mono"] {
            map.insert(iterator, read_frame(iterator, &library, frame).unwrap());
        }
    }
    if frame.id() == 0x183FCCE8 {
        for iterator in &["state_current_num", "soc", "voltage_stack", "current_system"] {
            map.insert(iterator, read_frame(iterator, &library, frame).unwrap());
        }
        // std::mem::swap(&mut values.current_system, &mut read_frame("current_system", &library, frame.data()));
        // configs::SystemValues::print_frame(values);
    }
    if frame.id() == 0x1840CCE8 {
        for iterator in &["current_hall", "current_shunt", "power_cumulative", "power_instant"] {
            map.insert(iterator, read_frame(iterator, &library, frame).unwrap());
        }
    }
    if frame.id() == 0x183CCCE8 {
        for iterator in &["voltage_hv1"] {
            map.insert(iterator, read_frame(iterator, &library, frame).unwrap());
        }
    }
    if frame.id() == 0x1834CCE8 {
    }
}
