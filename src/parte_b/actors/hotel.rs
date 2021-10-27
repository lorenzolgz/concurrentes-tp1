extern crate actix;
use crate::messages::entry_hotel_message::EntryHotelMessage;
use crate::messages::entry_hotel_success::EntryHotelSuccess;
use actix::{Actor, Handler, SyncContext};
use common::helper::fake_sleep;
use rand::{thread_rng, Rng};

pub struct Hotel {
    pub(crate) id: usize,
}

impl Actor for Hotel {
    type Context = SyncContext<Self>;
}

impl Handler<EntryHotelMessage> for Hotel {
    type Result = ();
    fn handle(&mut self, msg: EntryHotelMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("[HOTEL {}] recibi entry", self.id);
        fake_sleep(thread_rng().gen_range(5000..7000));
        println!("[HOTEL {}] contesto success", self.id);
        msg.sender
            .unwrap()
            .try_send(EntryHotelSuccess { id: self.id })
            .unwrap();
    }
}
