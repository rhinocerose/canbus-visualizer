#![allow(dead_code)]

use std::collections::HashMap;
use chrono::prelude::*;

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

pub struct StateStatus {
    state_current: States,
    state_previous: States,
    state_historic: States,
}

#[derive(Debug, Clone)]
pub struct NodeValue<'a> {
    pub identifier: &'a str,
    pub display_name: String,
    pub value: f32,
    pub frame_id: u32,
    pub last_updated: DateTime<Local>,
    pub frames_since_update: i32,
}

impl<'a> NodeValue<'a> {
    pub fn new(identifier: &'a str, display_name: String, frame_id: u32) -> NodeValue {
        NodeValue {
            identifier,
            display_name,
            value: 0.0,
            frame_id,
            last_updated: Local::now(),
            frames_since_update: -1,
        }
    }

    fn update_value(&mut self, value: f32) {
        self.value = value;
        self.frames_since_update = 0;
        self.last_updated = Local::now();
    }

    fn get_identifier(&self) -> &'a str {
        self.identifier
    }

    fn not_updated(&mut self) {
        self.frames_since_update += 1;
    }

    fn get_frame_id(&self) -> u32 {
        self.frame_id
    }
}

#[derive(Debug, Clone)]
pub struct Overview<'a> {
    pub hash_map: HashMap<&'a str, NodeValue<'a>>,
}

impl<'a> Overview<'a> {
    pub fn new() -> Overview<'a> {
        Overview { hash_map: HashMap::new() }
    }

    pub fn join(&mut self, values: NodeValue<'a>) {
        self.hash_map.
            insert(values.identifier, values);
    }

    pub fn update_entry(&mut self, identifier: &'a str, new_entry: f32) {
        self.hash_map.
            get_mut(identifier).unwrap().
            update_value(new_entry);
    }

    pub fn increment(&mut self) {
        for (_, val) in self.hash_map.iter_mut() {
            val.not_updated();
        }
    }

    pub fn match_frame(&self, frame_id: u32) -> Vec<&'a str> {
        let mut temp: Vec<&str> = Vec::new();
        for (_, val) in self.hash_map.iter() {
            if frame_id == val.get_frame_id() {
                temp.push(val.get_identifier());
            }
        }
        temp
    }
}
