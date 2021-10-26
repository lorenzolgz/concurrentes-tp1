extern crate actix;

use crate::actors::aeroservice::AeroService;
use crate::actors::hotel::Hotel;
use actix::{
    Actor, ActorFutureExt, AsyncContext, Context, Handler, ResponseActFuture, WrapFuture,
};
use actix::Addr;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::collections::HashMap;
use actix::clock::sleep;
use std::time::{Duration, SystemTime};
use crate::actors::entry_recipient::EntryRecipient;
use crate::messages::entry_aero_success::EntryAeroSuccess;
use crate::messages::entry_failed::EntryFailed;
use crate::messages::entry_hotel_message::EntryHotelMessage;
use crate::messages::entry_hotel_success::EntryHotelSuccess;
use crate::messages::entry_message::EntryMessage;

pub struct Orquestador {
    pub(crate) aeroservices: HashMap<String, Addr<AeroService>>,
    pub(crate) hotel: Addr<Hotel>,
}

impl Actor for Orquestador {
    type Context = Context<Self>;
}

impl Handler<EntryMessage> for Orquestador {
    type Result = ();

    fn handle(&mut self, _msg: EntryMessage, _ctx: &mut Context<Self>) -> Self::Result {
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
                aero_service.do_send(EntryMessage {
                    aero_id: _msg.aero_id,
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

impl Handler<EntryAeroSuccess> for Orquestador {
    type Result = ();

    fn handle(&mut self, msg: EntryAeroSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[Orquestador] recibí success de AEROSERVICE {}",
            msg.aero_id
        );
        if msg.original_message.is_hotel {
            self.hotel
                .try_send(EntryHotelMessage {
                    sender: Some(Arc::from(_ctx.address().recipient())),
                })
                .unwrap()
        }
    }
}

impl Handler<EntryFailed> for Orquestador {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: EntryFailed, _ctx: &mut Context<Self>) -> Self::Result {
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
                msg.aero_reference.try_send(EntryMessage{
                    aero_id: msg.aero_id,
                    is_hotel: msg.original_message.is_hotel,
                    sender: msg.original_message.sender.clone()
                }).unwrap()
            }))
    }
}

impl Handler<EntryHotelSuccess> for Orquestador {
    type Result = ();

    fn handle(&mut self, msg: EntryHotelSuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[Orquestador] recibí success de HOTEL {}", msg.id);
    }
}
