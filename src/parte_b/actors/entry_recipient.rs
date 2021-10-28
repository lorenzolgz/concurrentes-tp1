extern crate actix;

use crate::messages::aero_success::AeroSuccess;
use crate::messages::aero_failed::AeroFailed;
use actix::Recipient;

pub struct EntryRecipient {
    pub(crate) sender_success: Recipient<AeroSuccess>,
    pub(crate) sender_failed: Recipient<AeroFailed>,
}
