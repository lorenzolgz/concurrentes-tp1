extern crate actix;
use crate::messages::hotel_entry::HotelEntry;
use crate::messages::hotel_success::HotelSuccess;
use actix::{Actor, Addr, Handler, SyncContext};
use common::helper::fake_sleep;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use crate::actors::logger::Logger;
use crate::messages::log_message::LogMessage;

pub struct HotelService {
    pub(crate) logger: Arc<Addr<Logger>>,
}

impl Actor for HotelService {
    type Context = SyncContext<Self>;
}

impl Handler<HotelEntry> for HotelService {
    type Result = ();
    fn handle(&mut self, msg: HotelEntry, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.logger.do_send(LogMessage{
            log_entry: ("[HOTEL] recibi entry").to_string(),
        });
        fake_sleep(thread_rng().gen_range(5000..7000));
        self.logger.do_send(LogMessage{
            log_entry: ("[HOTEL] contesto success").to_string(),
        });
        msg.sender
            .do_send(HotelSuccess {
                original_origin: msg.original_origin,
                original_destination: msg.original_destination,
                elapsed_time: match msg.original_start_time.elapsed() {
                    Ok(duration) => {
                        Option::Some(duration)
                    }
                    Err(error) => {
                        println!("[HOTEL] Unable to calculate duration while replying to an HotelEntry, got error {}", error);
                        Option::None
                    }
                },
            })
            .unwrap_or_else(|error| {
                println!(
                    "[HOTEL] Unable to send HotelSuccess back to sender, got error {}",
                    error
                );
            });
    }
}
