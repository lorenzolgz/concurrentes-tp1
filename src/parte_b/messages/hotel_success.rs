extern crate actix;

use actix::Message;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct HotelSuccess {
    pub(crate) elapsed_time: Duration,
}
