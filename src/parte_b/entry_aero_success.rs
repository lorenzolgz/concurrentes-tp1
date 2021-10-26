extern crate actix;

use crate::entry_message::EntryMessage;
use actix::{
    Message,
};
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct EntryAeroSuccess {
    pub(crate) original_message: Arc<EntryMessage>,
    pub(crate) aero_id: usize,
}