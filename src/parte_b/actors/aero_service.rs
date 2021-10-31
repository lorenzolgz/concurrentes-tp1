extern crate actix;
use crate::actors::logger::Logger;
use crate::messages::aero_failed::AeroFailed;
use crate::messages::aero_success::AeroSuccess;
use crate::messages::entry::Entry;
use crate::messages::log_message::LogMessage;
use actix::{Actor, Addr, Handler, SyncContext};
use common::helper::fake_sleep;
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct AeroService {
    pub(crate) id: String,
    pub(crate) logger: Arc<Addr<Logger>>,
}

impl Actor for AeroService {
    type Context = SyncContext<Self>;
}

impl Handler<Entry> for AeroService {
    type Result = ();
    fn handle(&mut self, msg: Entry, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: ("[AEROSERVICE ".to_string()
                + &self.id.to_string()
                + &"] recibo entry".to_string()),
        });
        fake_sleep(thread_rng().gen_range(5000..7000));
        let is_success = thread_rng().gen_bool(0.5);

        self.logger.do_send(LogMessage {
            log_entry: ("[AEROSERVICE ".to_string()
                + &self.id.to_string()
                + &"] contesto is_success=".to_string()
                + &is_success.to_string()),
        });

        let orchestrator = msg.sender.clone();
        let elapsed_time = msg.start_time.elapsed();
        if is_success {
            orchestrator
                .try_send(AeroSuccess {
                    aero_id: self.id.to_string(),
                    original_message: Arc::from(msg),
                    elapsed_time: elapsed_time.map_or_else(
                        |error| {
                            println!(
                                "[AEROSERVICE {}] Unable to calculate elapsed time, got error {}",
                                self.id, error
                            );
                            Option::None
                        },
                        Option::Some,
                    ),
                })
                .unwrap_or_else(|error| {
                    println!(
                        "[AEROSERVICE {}] Unable to send AeroSuccess back to sender, got error {}",
                        self.id, error
                    );
                });
        } else {
            orchestrator
                .try_send(AeroFailed {
                    original_message: Arc::from(msg),
                    aero_reference: _ctx.address().recipient(),
                    aero_id: self.id.to_string(),
                })
                .unwrap_or_else(|error| {
                    println!(
                        "[AEROSERVICE {}] Unable to send AeroFailed back to sender, got error {}",
                        self.id, error
                    );
                });
        }
    }
}
