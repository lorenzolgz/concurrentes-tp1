extern crate actix;

use crate::messages::entry_message::EntryMessage;
use actix::Message;
use std::sync::Arc;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct EntryAeroSuccess {
    pub(crate) original_message: Arc<EntryMessage>,
    pub(crate) aero_id: String,
    pub(crate) elapsed_time: Duration,
}
