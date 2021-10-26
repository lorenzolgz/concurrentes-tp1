extern crate actix;

use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct EntryHotelSuccess {
    pub(crate) id: usize,
}
