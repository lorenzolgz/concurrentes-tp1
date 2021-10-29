extern crate actix;
use crate::messages::hotel_entry::HotelEntry;
use crate::messages::hotel_success::HotelSuccess;
use actix::{Actor, Handler, SyncContext};
use common::helper::fake_sleep;
use rand::{thread_rng, Rng};

pub struct HotelService {}

impl Actor for HotelService {
    type Context = SyncContext<Self>;
}

impl Handler<HotelEntry> for HotelService {
    type Result = ();
    fn handle(&mut self, msg: HotelEntry, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("[HOTEL] recibi entry");
        fake_sleep(thread_rng().gen_range(5000..7000));
        println!("[HOTEL] contesto success");
        match msg.original_start_time.elapsed() {
            Ok(duration) => {
                msg.sender
                    .do_send(HotelSuccess {
                        elapsed_time: duration,
                        original_origin: msg.original_origin,
                        original_destination: msg.original_destination,
                    })
                    .unwrap_or_else(|error| {
                        println!(
                            "[HOTEL] Unable to send HotelSuccess back to sender, got error {}",
                            error
                        );
                    });
            }
            Err(error) => {
                println!("[HOTEL] Unable to calculate duration while replying to an HotelEntry, got error {}",
                         error);
            }
        }
    }
}
