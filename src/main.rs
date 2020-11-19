mod configs;

use std::collections::HashMap;
use canparse::pgn::{PgnLibrary, SpnDefinition, ParseMessage};
use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};
use socketcan::{CANFrame};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut sys_val = configs::SystemValues::new();
    let mut values: HashMap<&str, f32> = HashMap::new();
    let lib = PgnLibrary::from_dbc_file("./data/anz.dbc").unwrap();
    let mut socket_rx = CANSocket::open("vcan0")?;

    while let Some(Ok(frame)) = socket_rx.next().await {
        parse_frame(&lib, &frame, &mut sys_val);
        //    configs::SystemValues::print_frame(&sys_val);
        // println!("{:?}", sys_val);
        // socket_tx.write_frame(frame)?.await;
    }
    Ok(())
}

fn read_frame(target_value: &str, library: &PgnLibrary, message: &[u8]) -> f32 {
    println!("{:?}, {:?}", message, target_value);
    let temp: &SpnDefinition = library.get_spn(target_value).unwrap();
    temp.parse_message(message).unwrap()
}

fn parse_frame(library: &PgnLibrary, frame: &CANFrame, values: &mut configs::SystemValues) {
    //println!("{:?}", frame);
    if frame.id() == 0x183ECCE8 {
        std::mem::swap(&mut values.temperature_diode, &mut read_frame("temperature_diode", &library, frame.data()));
        std::mem::swap(&mut values.temperature_contactor, &mut read_frame("temperature_contactor", &library, frame.data()));
        std::mem::swap(&mut values.temperature_max_mono, &mut read_frame("temperature_max_mono", &library, frame.data()));
        std::mem::swap(&mut values.temperature_min_mono, &mut read_frame("temperature_min_mono", &library, frame.data()));
        std::mem::swap(&mut values.voltage_max_mono, &mut read_frame("voltage_max_mono", &library, frame.data()));
        std::mem::swap(&mut values.voltage_min_mono, &mut read_frame("voltage_min_mono", &library, frame.data()));
        configs::SystemValues::print_frame(values);
    }
    if frame.id() == 0x183FCCE8 {
        std::mem::swap(&mut values.state_current_num, &mut read_frame("state_current_num", &library, frame.data()));
        std::mem::swap(&mut values.soc, &mut read_frame("soc", &library, frame.data()));
        std::mem::swap(&mut values.voltage_stack, &mut read_frame("voltage_stack", &library, frame.data()));
        std::mem::swap(&mut values.current_system, &mut read_frame("current_system", &library, frame.data()));
        configs::SystemValues::print_frame(values);
    }
    if frame.id() == 0x1840CCE8 {
        std::mem::swap(&mut values.current_hall, &mut read_frame("current_hall", &library, frame.data()));
        std::mem::swap(&mut values.current_shunt, &mut read_frame("current_shunt", &library, frame.data()));
        std::mem::swap(&mut values.power_cumulative, &mut read_frame("power_cumulative", &library, frame.data()));
        std::mem::swap(&mut values.power_instant, &mut read_frame("power_instant", &library, frame.data()));
        configs::SystemValues::print_frame(values);
    }
    if frame.id() == 0x183CCCE8 {
        std::mem::swap(&mut values.voltage_hv1, &mut read_frame("voltage_hv1", &library, frame.data()));
        std::mem::swap(&mut values.voltage_hv2, &mut read_frame("voltage_hv2", &library, frame.data()));
        std::mem::swap(&mut values.voltage_supply, &mut read_frame("voltage_supply", &library, frame.data()));
    }
    if frame.id() == 0x1834CCE8 {
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        println!("FOUND FRAME");
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        std::mem::swap(&mut values.raw_soc, &mut read_frame("raw_soc", &library, frame.data()));
    }
}
