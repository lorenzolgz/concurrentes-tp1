extern crate actix;

use actix::Message;
use std::time::Duration;

/// A message representation of a (Successful) Hotel Response, original origin and destination are kept
/// for reporting and metric purposes
#[derive(Message)]
#[rtype(result = "()")]
pub struct HotelSuccess {
    pub(crate) elapsed_time: Option<Duration>,
    pub(crate) original_origin: String,
    pub(crate) original_destination: String,
}

impl HotelSuccess {
    /// Used to describe the message across logs
    pub fn describe(&self) -> String {
        format!(
            "Origin: {}, Destination: {}",
            self.original_origin, self.original_destination
        )
    }
}
