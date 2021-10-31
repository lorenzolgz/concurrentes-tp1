extern crate actix;

use crate::actors::orchestrator::Orchestrator;
use actix::{Addr, Message};
use std::time::SystemTime;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Entry {
    pub(crate) aero_id: String,
    pub(crate) origin: String,
    pub(crate) destination: String,
    pub(crate) includes_hotel: bool,
    pub(crate) sender: Addr<Orchestrator>,
    pub(crate) start_time: SystemTime,
}

impl Entry {
    pub fn describe(&self) -> String {
        format!(
            "Origin: {}, Destination: {}, Airline: {}, Package: {}",
            self.origin, self.destination, self.aero_id, self.includes_hotel
        )
    }
}
