extern crate actix;

use crate::messages::hotel_success::HotelSuccess;
use actix::{Message, Recipient};
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Message)]
#[rtype(result = "()")]
pub struct HotelEntry {
    pub(crate) sender: Arc<Recipient<HotelSuccess>>,
    pub(crate) original_origin: String,
    pub(crate) original_destination: String,
    pub(crate) original_start_time: SystemTime,
}
