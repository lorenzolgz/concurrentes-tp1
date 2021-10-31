extern crate actix;

use actix::Message;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct HotelSuccess {
    pub(crate) elapsed_time: Option<Duration>,
    pub(crate) original_origin: String,
    pub(crate) original_destination: String,
}

impl HotelSuccess {
    pub fn describe(&self) -> String {
        format!(
            "Origin: {}, Destination: {}",
            self.original_origin, self.original_destination
        )
    }
}
