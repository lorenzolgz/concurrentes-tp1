extern crate actix;

use crate::actors::orchestrator::Orchestrator;
use actix::{Addr, Message};
use std::time::SystemTime;

/// A message representation of a CSV entry, that also holds the address of the main actor
/// orchestrator and the start_time at which the request was first processed
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
    /// Used to describe the message across logs
    pub fn describe(&self) -> String {
        format!(
            "Origin: {}, Destination: {}, Airline: {}, Package: {}",
            self.origin, self.destination, self.aero_id, self.includes_hotel
        )
    }
}
