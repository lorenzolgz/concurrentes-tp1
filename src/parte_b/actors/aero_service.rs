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

/// Struct Representing an Airline's WebService
/// Its id identifies the airline and the logger reference is kept for logging purposes
pub struct AeroService {
    pub(crate) id: String,
    pub(crate) logger: Arc<Addr<Logger>>,
}

/// It's an Actor with SyncContext as it is instanced N times via SyncArbiter
impl Actor for AeroService {
    type Context = SyncContext<Self>;
}

/// Handler of an Entry, that ends up replying either an AeroSuccess or an AeroFailed
/// back to the Orchestrator
impl Handler<Entry> for AeroService {
    type Result = ();
    fn handle(&mut self, msg: Entry, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: ("[AEROSERVICE ".to_string()
                + &self.id.to_string()
                + "] Got Entry Message || "
                + &msg.describe()),
        });
        fake_sleep(thread_rng().gen_range(5000..7000));
        let is_success = thread_rng().gen_bool(0.5);

        self.logger.do_send(LogMessage {
            log_entry: ("[AEROSERVICE ".to_string()
                + &self.id.to_string()
                + "] For Entry Message || "
                + &msg.describe()
                + " || will reply with is_success="
                + &is_success.to_string()),
        });

        let orchestrator = msg.sender.clone();
        let elapsed_time = msg.start_time.elapsed();
        let ref_msg = Arc::from(msg);
        if is_success {
            orchestrator
                .try_send(AeroSuccess {
                    original_message: ref_msg.clone(),
                    elapsed_time: elapsed_time.map_or_else(
                        |error| {
                            self.logger.do_send(LogMessage {
                                log_entry: ("[AEROSERVICE ".to_string()
                                    + &self.id.to_string()
                                    + &"] Unable to calculate elapsed time, got error".to_string()
                                    + &error.to_string()),
                            });
                            Option::None
                        },
                        Option::Some,
                    ),
                })
                .unwrap_or_else(|error| {
                    self.logger.do_send(LogMessage {
                        log_entry: ("[AEROSERVICE ".to_string()
                            + &self.id.to_string()
                            + "] For Entry Message || "
                            + &ref_msg.describe()
                            + " Unable to send AeroSuccess back to sender, got error"
                            + &error.to_string()),
                    });
                });
        } else {
            orchestrator
                .try_send(AeroFailed {
                    original_message: ref_msg.clone(),
                    aero_reference: _ctx.address().recipient(),
                    aero_id: self.id.to_string(),
                })
                .unwrap_or_else(|error| {
                    self.logger.do_send(LogMessage {
                        log_entry: ("[AEROSERVICE ".to_string()
                            + &self.id.to_string()
                            + "] For Entry Message || "
                            + &ref_msg.describe()
                            + " Unable to send AeroFailed back to sender, got error"
                            + &error.to_string()),
                    });
                });
        }
    }
}
