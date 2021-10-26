extern crate actix;
use crate::messages::entry_hotel_message::EntryHotelMessage;
use crate::messages::entry_hotel_success::EntryHotelSuccess;
use actix::{Actor, Handler, SyncContext};

pub struct Hotel {
    pub(crate) id: usize,
}

impl Actor for Hotel {
    type Context = SyncContext<Self>;
}

impl Handler<EntryHotelMessage> for Hotel {
    type Result = ();
    fn handle(&mut self, msg: EntryHotelMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("[HOTEL {}] recibo entry, contesto success", self.id);
        msg.sender
            .unwrap()
            .try_send(EntryHotelSuccess { id: self.id })
            .unwrap();
    }
}
