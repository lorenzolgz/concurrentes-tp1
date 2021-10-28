extern crate actix;

use crate::messages::entry::Entry;
use actix::{Message, Recipient};
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct AeroFailed {
    pub(crate) original_message: Arc<Entry>,
    pub(crate) aero_reference: Recipient<Entry>,
    pub(crate) aero_id: String,
}
