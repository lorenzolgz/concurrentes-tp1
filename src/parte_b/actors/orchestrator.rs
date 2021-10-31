extern crate actix;

use crate::actors::aero_service::AeroService;
use crate::actors::benchmark::Benchmark;
use crate::actors::hotel_service::HotelService;
use crate::actors::logger::Logger;
use crate::messages::aero_failed::AeroFailed;
use crate::messages::aero_success::AeroSuccess;
use crate::messages::entry::Entry;
use crate::messages::hotel_entry::HotelEntry;
use crate::messages::hotel_success::HotelSuccess;
use crate::messages::log_message::LogMessage;
use crate::messages::request_completed::RequestCompleted;
use actix::clock::sleep;
use actix::Addr;
use actix::{Actor, ActorFutureExt, AsyncContext, Context, Handler, ResponseActFuture, WrapFuture};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// Main Actor of the service, is in charge of handling an Entry's flow from the beginning
/// by forwarding to the AeroService, later on to the Hotel (if applicable) and at last
/// to the Benchmark for metric purposes
pub struct Orchestrator {
    pub(crate) aeroservices: HashMap<String, Addr<AeroService>>,
    pub(crate) hotel: Addr<HotelService>,
    pub(crate) benchmark: Addr<Benchmark>,
    pub(crate) logger: Addr<Logger>,
}

impl Actor for Orchestrator {
    type Context = Context<Self>;
}

/// Forwards Entry message to the required airline AeroService
/// If such airline is not found, the message is ignored
impl Handler<Entry> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: Entry, _ctx: &mut Context<Self>) -> Self::Result {
        let airline_key = msg.aero_id.clone();
        self.logger.do_send(LogMessage {
            log_entry: ("[ORCHESTRATOR] Got Entry Message || ".to_string() + &msg.describe()),
        });
        self.aeroservices.get(&airline_key).map_or_else(
            || {
                self.logger.do_send(LogMessage {
                    log_entry: "[ORCHESTRATOR] Unable to find aeroservice for airline ".to_string()
                        + &airline_key,
                });
            },
            |aero_service| aero_service.do_send(msg),
        );
    }
}
/// Depending on the original Entry it will either
/// send HotelEntry to Hotel or send RequestCompleted to Benchmark
impl Handler<AeroSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: AeroSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: "[ORCHESTRATOR] Got AeroSuccess Message || ".to_string() + &msg.describe(),
        });

        if msg.original_message.includes_hotel {
            self.hotel.do_send(HotelEntry {
                sender: Arc::from(_ctx.address().recipient()),
                original_start_time: SystemTime::now(),
                original_origin: msg.original_message.origin.to_string(),
                original_destination: msg.original_message.destination.to_string(),
            })
        } else {
            self.benchmark.do_send(RequestCompleted {
                origin: msg.original_message.origin.to_string(),
                destination: msg.original_message.destination.to_string(),
                time_elapsed: msg.elapsed_time,
            })
        }
    }
}

/// Will wait a random amount of time and will re-send the original Entry back to the
/// required AeroService
impl Handler<AeroFailed> for Orchestrator {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: AeroFailed, _ctx: &mut Context<Self>) -> Self::Result {
        let millis_to_sleep = thread_rng().gen_range(500..2000);
        let timer = SystemTime::now();
        self.logger.do_send(LogMessage {
            log_entry: "[ORCHESTRATOR] Got AeroFailed Message || ".to_string()
                + &msg.describe()
                + ", will retry requesting in "
                + &millis_to_sleep.to_string()
                + " millis",
        });

        Box::pin(sleep(Duration::from_millis(millis_to_sleep))
            .into_actor(self)
            .map(move |_result, me, _ctx| {
                match timer.elapsed() {
                    Ok(duration) => {
                        me.logger.do_send(LogMessage{
                            log_entry: ("[ORCHESTRATOR] Will retry pending request after ".to_string() + &duration.as_millis().to_string() +
                                &" (asked for: ".to_string() + &millis_to_sleep.to_string() + &") to AEROSERVICE ".to_string() +
                            &msg.aero_id),
                        });

                    }
                    Err(error) => {
                        me.logger.do_send(LogMessage{
                            log_entry: ("[ORCHESTRATOR] Unable to calculate duration while replying to AEROSERVICE ".to_string() +
                                &msg.aero_id + &", got error ".to_string() + &error.to_string()),
                        });
                    }
                }
                msg.aero_reference.do_send(Entry {
                    aero_id: msg.aero_id,
                    origin: msg.original_message.origin.to_string(),
                    destination: msg.original_message.destination.to_string(),
                    start_time: msg.original_message.start_time,
                    includes_hotel: msg.original_message.includes_hotel,
                    sender: msg.original_message.sender.clone()
                }).unwrap_or_else(|error| {
                    me.logger.do_send(LogMessage{
                        log_entry: ("[ORCHESTRATOR] Unable to send Entry to AeroService, got error ".to_string() +
                            &error.to_string()),
                    });
                })
            }))
    }
}

/// Will send a RequestCompleted to Benchmark
impl Handler<HotelSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: HotelSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: ("[ORCHESTRATOR] Got HotelSuccess Message || ".to_string()
                + &msg.describe()),
        });
        self.benchmark.do_send(RequestCompleted {
            time_elapsed: msg.elapsed_time,
            origin: msg.original_origin,
            destination: msg.original_destination,
        })
    }
}
