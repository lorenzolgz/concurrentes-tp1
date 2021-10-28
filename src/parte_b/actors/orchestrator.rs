extern crate actix;

use crate::actors::aero_service::AeroService;
use crate::actors::benchmark::Benchmark;
use crate::actors::entry_recipient::EntryRecipient;
use crate::actors::hotel_service::HotelService;
use crate::messages::aero_success::AeroSuccess;
use crate::messages::aero_failed::AeroFailed;
use crate::messages::hotel_entry::HotelEntry;
use crate::messages::hotel_success::HotelSuccess;
use crate::messages::entry::Entry;
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

    fn handle(&mut self, _msg: Entry, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[Orquestador] recibi entry message de aeropuerto {}",
            _msg.aero_id
        );
        self.aeroservices.get(&_msg.aero_id).map_or_else(
            || {
                println!("[Orquestador] Unable to find aeroservice for an airline")
                // TODO imprimir tambien el aero_id
            },
            |aero_service| {
                aero_service.do_send(Entry {
                    aero_id: _msg.aero_id,
                    start_time: _msg.start_time,
                    is_hotel: _msg.is_hotel,
                    sender: Option::Some(Arc::from(EntryRecipient {
                        sender_failed: _ctx.address().recipient(),
                        sender_success: _ctx.address().recipient(),
                    })),
                })
            },
        )
    }
}

impl Handler<AeroSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: AeroSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[Orquestador] recibí success de AEROSERVICE {}",
            msg.aero_id
        );
        if msg.original_message.is_hotel {
            self.hotel
                .try_send(HotelEntry {
                    sender: Some(Arc::from(_ctx.address().recipient())),
                    original_start_time: SystemTime::now(),
                })
                .unwrap()
        } else {
            self.benchmark.do_send(RequestCompleted {
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
                println!("[Orquestador] me desperté despues de {}/{} millis para contestarle a AEROSERVICE {}",
                         timer.elapsed().unwrap().as_millis(),
                         millis_to_sleep,msg.aero_id);
                msg.aero_reference.try_send(Entry {
                    aero_id: msg.aero_id,
                    start_time: msg.original_message.start_time,
                    is_hotel: msg.original_message.is_hotel,
                    sender: msg.original_message.sender.clone()
                }).unwrap()
            }))
    }
}

impl Handler<HotelSuccess> for Orchestrator {
    type Result = ();

    fn handle(&mut self, msg: HotelSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[Orquestador] recibí success de HOTEL");
        self.benchmark.do_send(RequestCompleted {
            time_elapsed: msg.elapsed_time,
        })
    }
}
