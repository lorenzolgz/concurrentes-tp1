extern crate actix;

use crate::messages::hotel_success::HotelSuccess;
use actix::{Message, Recipient};
use std::sync::Arc;
use std::time::SystemTime;

/// A message representation of an Hotel Request, original origin and destination are kept
/// for reporting and metric purposes
#[derive(Message)]
#[rtype(result = "()")]
pub struct HotelEntry {
    pub(crate) sender: Arc<Recipient<HotelSuccess>>,
    pub(crate) original_origin: String,
    pub(crate) original_destination: String,
    pub(crate) original_start_time: SystemTime,
}

impl HotelEntry {
    /// Used to describe the message across logs
    pub fn describe(&self) -> String {
        format!(
            "Origin: {}, Destination: {}",
            self.original_origin, self.original_destination
        )
    }
}
