extern crate actix;

use crate::entry_hotel_success::EntryHotelSuccess;
use actix::{Message, Recipient};
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct EntryHotelMessage {
    pub(crate) sender: Option<Arc<Recipient<EntryHotelSuccess>>>,
}
