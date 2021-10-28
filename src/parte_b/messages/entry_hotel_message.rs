extern crate actix;

use crate::messages::entry_hotel_success::EntryHotelSuccess;
use actix::{Message, Recipient};
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Message)]
#[rtype(result = "()")]
pub struct EntryHotelMessage {
    pub(crate) sender: Option<Arc<Recipient<EntryHotelSuccess>>>,
    pub(crate) original_start_time: SystemTime,
}
