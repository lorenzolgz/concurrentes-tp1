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

pub struct Orchestrator {
    pub(crate) aeroservices: HashMap<String, Addr<AeroService>>,
    pub(crate) hotel: Addr<HotelService>,
    pub(crate) benchmark: Addr<Benchmark>,
    pub(crate) logger: Addr<Logger>,
}

impl Actor for Orchestrator {
    type Context = Context<Self>;
}

impl Handler<Entry> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: Entry, _ctx: &mut Context<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: ("[Orquestador] recibi entry message de aeropuerto ".to_string()
                + &msg.aero_id.to_string()),
        });
        self.aeroservices.get(&msg.aero_id).map_or_else(
            || {
                self.logger.do_send(LogMessage {
                    log_entry: ("[Orquestador] Unable to find aeroservice for an airline "
                        .to_string()),
                });
                // TODO imprimir tambien el aero_id
            },
            |aero_service| aero_service.do_send(msg),
        );
    }
}

impl Handler<AeroSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: AeroSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: ("[Orquestador] recibí success de AEROSERVICE ".to_string()
                + &msg.aero_id.to_string()),
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

impl Handler<AeroFailed> for Orchestrator {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: AeroFailed, _ctx: &mut Context<Self>) -> Self::Result {
        let millis_to_sleep = thread_rng().gen_range(500..2000);
        let timer = SystemTime::now();
        self.logger.do_send(LogMessage {
            log_entry: ("[Orquestador] recibí failed de AEROSERVICE ".to_string()
                + &msg.aero_id.to_string()
                + &", me voy a dormir ".to_string()
                + &millis_to_sleep.to_string()
                + &" millis".to_string()),
        });

        Box::pin(sleep(Duration::from_millis(millis_to_sleep))
            .into_actor(self)
            .map(move |_result, _me, _ctx| {
                match timer.elapsed() {
                    Ok(duration) => {
                        println!("[Orquestador] Woke up after {} (asked for: {}) to retry request to AEROSERVICE {}",
                                 duration.as_millis(),
                                 millis_to_sleep,
                                 msg.aero_id);

                        /*self.logger.do_send(LogMessage{
                            log_entry: ("[Orquestador] Woke up after ".to_string() + &duration.as_millis().to_string() +
                                &" (asked for: ".to_string() + &millis_to_sleep.to_string() + &") to retry request to AEROSERVICE ".to_string() +
                            &msg.aero_id.to_string()),
                        });*/

                    }
                    Err(error) => {
                        println!("[Orquestador] Unable to calculate duration while replying to AEROSERVICE {}, got error {}",
                                 msg.aero_id,
                                 error);

                        /*self.logger.do_send(LogMessage{
                            log_entry: ("[Orquestador] Unable to calculate duration while replying to AEROSERVICE ".to_string() +
                                &msg.aero_id.to_string() + &", got error ".to_string() + &error.to_string()),
                        });*/
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
                    println!("[Orquestador] Unable to send Entry to AeroService, got error {}", error);
                    /*self.logger.do_send(LogMessage{
                        log_entry: ("[Orquestador] Unable to send Entry to AeroService, got error ".to_string() +
                            &error.to_string()),
                    });*/
                })
            }))
    }
}

impl Handler<HotelSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: HotelSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        self.logger.do_send(LogMessage {
            log_entry: ("[Orquestador] recibi success de HOTEL".to_string()),
        });
        self.benchmark.do_send(RequestCompleted {
            time_elapsed: msg.elapsed_time,
            origin: msg.original_origin,
            destination: msg.original_destination,
        })
    }
}
