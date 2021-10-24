extern crate actix;

use actix::clock::sleep;
use actix::{
    Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler, Message, Recipient,
    ResponseActFuture, SyncArbiter, SyncContext, System, WrapFuture,
};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

const MESSAGES_PER_AERO: usize = 50;

#[derive(Message)]
#[rtype(result = "()")]
struct EntryMessage {
    aero_id: usize,
    sender: Option<Arc<EntryRecipient>>,
}

struct EntryRecipient {
    sender_success: Recipient<EntrySuccess>,
    sender_failed: Recipient<EntryFailed>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct EntrySuccess {
    aero_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
struct EntryFailed {
    original_message: Arc<EntryMessage>,
    aero_reference: Recipient<EntryMessage>,
    aero_id: usize,
}

struct Orquestador {
    aeroservices: HashMap<usize, Addr<AeroService>>,
}

struct AeroService {
    id: usize,
}

impl Actor for Orquestador {
    type Context = Context<Self>;
}

impl Actor for AeroService {
    type Context = SyncContext<Self>;
}

impl Handler<EntryMessage> for Orquestador {
    type Result = ();

    fn handle(&mut self, _msg: EntryMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[Orquestador] recibi entry message");
        self.aeroservices
            .get(&_msg.aero_id)
            .unwrap()
            .do_send(EntryMessage {
                aero_id: _msg.aero_id,
                sender: Option::Some(Arc::from(EntryRecipient {
                    sender_failed: _ctx.address().recipient(),
                    sender_success: _ctx.address().recipient(),
                })),
            });
    }
}

impl Handler<EntrySuccess> for Orquestador {
    type Result = ();

    fn handle(&mut self, msg: EntrySuccess, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[Orquestador] recibí success de AEROSERVICE {}",
            msg.aero_id
        );
    }
}

impl Handler<EntryFailed> for Orquestador {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: EntryFailed, _ctx: &mut Context<Self>) -> Self::Result {
        let millis_to_sleep = thread_rng().gen_range(500..2000);
        let timer = SystemTime::now();
        println!(
            "[Orquestador] recibí failed de AEROSERVICE {}, me voy a dormir {} millis",
            msg.aero_id,
            millis_to_sleep
        );
        Box::pin(sleep(Duration::from_millis(millis_to_sleep))
            .into_actor(self)
            .map(move |_result, _me, _ctx| {
                println!("[Orquestador] me desperté despues de {}/{} millis para contestarle a AEROSERVICE {}",
                         timer.elapsed().unwrap().as_millis(),
                         millis_to_sleep,msg.aero_id);
                msg.aero_reference.try_send(EntryMessage{
                    aero_id: msg.aero_id,
                    sender: msg.original_message.sender.clone()
                }).unwrap()
            }))
    }
}

impl Handler<EntryMessage> for AeroService {
    type Result = ();
    fn handle(&mut self, msg: EntryMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("[AEROSERVICE {}] recibo entry", self.id);
        let is_success = thread_rng().gen_bool(0.5);
        println!("[AEROSERVICE {}] is_success={}", self.id, is_success);
        if is_success {
            msg.sender
                .unwrap()
                .sender_success
                .try_send(EntrySuccess { aero_id: self.id })
                .unwrap();
        } else {
            let copy_msg = Arc::from(msg);
            copy_msg
                .sender
                .as_ref()
                .unwrap()
                .sender_failed
                .try_send(EntryFailed {
                    original_message: copy_msg.clone(),
                    aero_reference: _ctx.address().recipient(),
                    aero_id: self.id,
                })
                .unwrap();
        }
    }
}

fn main() {
    let system = System::new();
    system.block_on(async {
        let mut aeroservices = HashMap::new();
        aeroservices.insert(1, SyncArbiter::start(1, || AeroService { id: 1 }));
        aeroservices.insert(2, SyncArbiter::start(1, || AeroService { id: 2 }));
        aeroservices.insert(3, SyncArbiter::start(1, || AeroService { id: 3 }));

        let otro_orq = Arc::from(Orquestador { aeroservices }.start());

        for aero_id in 1..3 {
            for _i in 0..MESSAGES_PER_AERO {
                otro_orq.do_send(EntryMessage {
                    aero_id,
                    sender: Option::None,
                });
            }
        }
    });

    system.run().unwrap();
}