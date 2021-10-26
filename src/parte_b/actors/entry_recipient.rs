extern crate actix;

use crate::messages::entry_aero_success::EntryAeroSuccess;
use crate::messages::entry_failed::EntryFailed;
use actix::Recipient;

pub struct EntryRecipient {
    pub(crate) sender_success: Recipient<EntryAeroSuccess>,
    pub(crate) sender_failed: Recipient<EntryFailed>,
}
