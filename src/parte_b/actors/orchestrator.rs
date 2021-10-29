extern crate actix;

use crate::actors::aero_service::AeroService;
use crate::actors::benchmark::Benchmark;
use crate::actors::hotel_service::HotelService;
use crate::messages::aero_failed::AeroFailed;
use crate::messages::aero_success::AeroSuccess;
use crate::messages::entry::Entry;
use crate::messages::hotel_entry::HotelEntry;
use crate::messages::hotel_success::HotelSuccess;
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
}

impl Actor for Orchestrator {
    type Context = Context<Self>;
}

impl Handler<Entry> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: Entry, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[Orquestador] recibi entry message de aeropuerto {}",
            msg.aero_id
        );
        self.aeroservices.get(&msg.aero_id).map_or_else(
            || {
                println!("[Orquestador] Unable to find aeroservice for an airline")
                // TODO imprimir tambien el aero_id
            },
            |aero_service| aero_service.do_send(msg),
        );
    }
}

impl Handler<AeroSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: AeroSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[Orquestador] recibí success de AEROSERVICE {}",
            msg.aero_id
        );
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
        println!(
            "[Orquestador] recibí failed de AEROSERVICE {}, me voy a dormir {} millis",
            msg.aero_id, millis_to_sleep
        );
        Box::pin(sleep(Duration::from_millis(millis_to_sleep))
            .into_actor(self)
            .map(move |_result, _me, _ctx| {
                match timer.elapsed() {
                    Ok(duration) => {
                        println!("[Orquestador] Woke up after {} (asked for: {}) to retry request to AEROSERVICE {}",
                                 duration.as_millis(),
                                 millis_to_sleep,
                                 msg.aero_id);
                    }
                    Err(error) => {
                        println!("[Orquestador] Unable to calculate duration while replying to AEROSERVICE {}, got error {}",
                                 msg.aero_id,
                                 error);
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
                })
            }))
    }
}

impl Handler<HotelSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: HotelSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[Orquestador] recibí success de HOTEL");
        self.benchmark.do_send(RequestCompleted {
            time_elapsed: msg.elapsed_time,
            origin: msg.original_origin,
            destination: msg.original_destination,
        })
    }
}
