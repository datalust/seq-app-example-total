#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate serde;
extern crate serde_json;

use std::env;
use chrono::{DateTime, Utc};
use std::io;
use std::io::prelude::*;
use serde_json::{Value};

#[derive(Serialize)]
struct CurrentTotalEvent<'a> {
    #[serde(rename="@t")]
    timestamp: DateTime<Utc>,

    #[serde(rename="@mt")]
    message_template: &'static str,

    #[serde(rename="PropertyName")]
    property_name: &'a str,

    #[serde(rename="Total")]
    total: f64
}

const CURRENT_TOTAL_IS : &'static str = "The current total of {PropertyName} is {Total}";

impl<'a> CurrentTotalEvent<'a> {
    pub fn new<'b>(property_name: &'b str, total: f64) -> CurrentTotalEvent<'b> {
        CurrentTotalEvent {
            timestamp: Utc::now(),
            message_template: CURRENT_TOTAL_IS,
            property_name: property_name,
            total: total
        }
    }
}

fn emit_total<'a>(property_name: &'a str, total: f64) {
    let evt = CurrentTotalEvent::new(property_name, total);
    let json = serde_json::to_string(&evt).unwrap();
    eprintln!("{}", json);
}

fn main() {
    let property_name = env::var("SEQ_APP_SETTING_PROPERTYNAME").expect("Property name setting is required");
    let mut current_total: f64 = 0.;
    let stdin = io::stdin();

    emit_total(&property_name, current_total);

    for input in stdin.lock().lines() {
        let line = input.unwrap();
        let data: Value = serde_json::from_str(&line).unwrap();
        if let Some(f) = data.as_object()
                .and_then(|o| o.get(&property_name))
                .and_then(|n| n.as_f64()) {
            current_total += f;
            emit_total(&property_name, current_total);
        }
    }
}
