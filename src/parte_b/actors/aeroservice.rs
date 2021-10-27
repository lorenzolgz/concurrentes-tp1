extern crate actix;
use crate::messages::entry_aero_success::EntryAeroSuccess;
use crate::messages::entry_failed::EntryFailed;
use crate::messages::entry_message::EntryMessage;
use actix::{Actor, Handler, SyncContext};
use common::helper::fake_sleep;
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct AeroService {
    pub(crate) id: String,
}

impl Actor for AeroService {
    type Context = SyncContext<Self>;
}

impl Handler<EntryMessage> for AeroService {
    type Result = ();
    fn handle(&mut self, msg: EntryMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("[AEROSERVICE {}] recibo entry", self.id);
        fake_sleep(thread_rng().gen_range(5000..7000));
        let is_success = thread_rng().gen_bool(0.5);
        println!(
            "[AEROSERVICE {}] contesto is_success={}",
            self.id, is_success
        );
        let copy_msg = Arc::from(msg);
        let recipient = copy_msg.sender.as_ref().unwrap();
        if is_success {
            recipient
                .sender_success
                .try_send(EntryAeroSuccess {
                    aero_id: self.id.to_string(),
                    original_message: copy_msg.clone(),
                    elapsed_time: copy_msg.time.elapsed().unwrap(),
                })
                .unwrap()
        } else {
            recipient
                .sender_failed
                .try_send(EntryFailed {
                    original_message: copy_msg.clone(),
                    aero_reference: _ctx.address().recipient(),
                    aero_id: self.id.to_string(),
                })
                .unwrap()
        }
    }
}
