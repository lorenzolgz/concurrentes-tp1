extern crate actix;

use crate::actors::entry_recipient::EntryRecipient;
use actix::Message;
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct EntryMessage {
    pub(crate) aero_id: String,
    pub(crate) is_hotel: bool,
    pub(crate) sender: Option<Arc<EntryRecipient>>,
}
