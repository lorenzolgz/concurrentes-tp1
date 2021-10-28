extern crate actix;
use crate::messages::entry_hotel_message::EntryHotelMessage;
use crate::messages::entry_hotel_success::EntryHotelSuccess;
use actix::{Actor, Handler, SyncContext};
use common::helper::fake_sleep;
use rand::{thread_rng, Rng};

pub struct Hotel {}

impl Actor for Hotel {
    type Context = SyncContext<Self>;
}

impl Handler<EntryHotelMessage> for Hotel {
    type Result = ();
    fn handle(&mut self, msg: EntryHotelMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("[HOTEL] recibi entry");
        fake_sleep(thread_rng().gen_range(5000..7000));
        println!("[HOTEL] contesto success");
        msg.sender
            .unwrap()
            .try_send(EntryHotelSuccess {
                elapsed_time: msg.original_start_time.elapsed().unwrap(),
            })
            .unwrap();
    }
}
