extern crate actix;

use crate::messages::entry::Entry;
use actix::{Message, Recipient};
use std::sync::Arc;

/// A message representation of a (Failed) Aero Service Response, original message
/// and aero reference is kept to be able to retry
#[derive(Message)]
#[rtype(result = "()")]
pub struct AeroFailed {
    pub(crate) original_message: Arc<Entry>,
    pub(crate) aero_reference: Recipient<Entry>,
    pub(crate) aero_id: String,
}

impl AeroFailed {
    /// Used to describe the message across logs
    pub fn describe(&self) -> String {
        self.original_message.describe()
    }
}
