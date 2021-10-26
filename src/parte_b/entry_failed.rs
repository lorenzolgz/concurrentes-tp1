extern crate actix;

use crate::entry_message::EntryMessage;
use actix::{
    Message, Recipient,
};
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct EntryFailed {
    pub(crate) original_message: Arc<EntryMessage>,
    pub(crate) aero_reference: Recipient<EntryMessage>,
    pub(crate) aero_id: usize,
}