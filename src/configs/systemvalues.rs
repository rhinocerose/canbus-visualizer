#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum States {
    Standby,
    Charge,
    Discharge,
    EOD,
    Service,
    PreStandby,
    Error,
}

#[derive(Debug, Clone)]
pub struct CollectedValues<'a> {
    pub identifier: &'a str,
    pub display_name: String,
    pub value: f32,
    pub last_updated: i32,
}

impl<'a> CollectedValues<'a> {
    pub fn new(identifier: &'a str, display_name: String) -> CollectedValues {
        CollectedValues {
            identifier,
            display_name,
            value: 0.0,
            last_updated: -1,
        }
    }

    pub fn update_entry(&mut self, value: f32) {
        self.value = value;
        self.last_updated = 0;
    }

    pub fn give_id(&self) -> &'a str {
        self.identifier
    }

    pub fn not_updated(&mut self) {
        self.last_updated += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Overview<'a> {
    pub hash_map: HashMap<&'a str, CollectedValues<'a>>,
}

impl<'a> Overview<'a> {
    pub fn new() -> Overview<'a> {
        Overview { hash_map: HashMap::new() }
    }

    pub fn join(&mut self, values: CollectedValues<'a>) {
        self.hash_map.insert(values.identifier, values);
    }

    pub fn update_entry(&mut self, identifier: &'a str, new_entry: f32) {
        let temp = self.hash_map.get_mut(identifier).unwrap();
        temp.update_entry(new_entry);
    }

}

#[derive(Debug, Copy, Clone)]
pub struct SystemValues {
    pub temperature_contactor: f32,
    pub temperature_diode: f32,
    pub temperature_max_mono: f32,
    pub temperature_min_mono: f32,
    pub voltage_max_mono: f32,
    pub voltage_min_mono: f32,
    pub voltage_hv1: f32,
    pub voltage_hv2: f32,
    pub voltage_supply: f32,
    pub state_current_num: f32,
    pub state_current: States,
    pub soc: f32,
    pub voltage_stack: f32,
    pub current_system: f32,
    pub power_instant: f32,
    pub power_cumulative: f32,
    pub current_shunt: f32,
    pub current_hall: f32,
    pub raw_temperature_diode: f32,
    pub raw_soc: f32,
}

impl SystemValues {
    pub fn new() -> SystemValues {
        SystemValues {
            temperature_contactor: 0.0,
            temperature_diode: 0.0,
            temperature_max_mono: 0.0,
            temperature_min_mono: 0.0,
            voltage_max_mono: 0.0,
            voltage_min_mono: 0.0,
            voltage_hv1: 0.0,
            voltage_hv2: 0.0,
            voltage_supply: 0.0,
            state_current_num: 0.0,
            state_current: States::Standby,
            soc: 0.0,
            voltage_stack: 0.0,
            current_system: 0.0,
            power_instant: 0.0,
            power_cumulative: 0.0,
            current_shunt: 0.0,
            current_hall: 0.0,
            raw_temperature_diode: 0.0,
            raw_soc: 0.0,
        }
    }
    pub fn print_frame(&self) {
        println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
        println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
        println!("%%  TEMPS:");
        println!("%%  Max Temp {}, Min Temp {}, Diode Temp {}, Contactor Temp {}",
            self.temperature_max_mono,
            self.temperature_min_mono,
            self.temperature_diode,
            self.temperature_contactor);
        println!("%%  VOLTAGES:");
        println!("%%  Max voltages {:.2}, Min voltages {:.2}, Stack voltage {:.2}",
            self.voltage_max_mono,
            self.voltage_min_mono,
            self.voltage_stack);
        println!("%%  CURRENTS:");
        println!("%%  System Current {:.2}, Hall Current {:.2}, Shunt Current {:.2}",
            self.current_system,
            self.current_hall,
            self.current_shunt);
        println!("%%  STATE: {}", self.state_current_num);
        println!("%%  SOC: {}/%", self.soc);
        println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
        println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    }
}

impl std::fmt::Display for SystemValues {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Contactor: {}, Diode: {}", self.temperature_contactor, self.temperature_diode)
    }
}
