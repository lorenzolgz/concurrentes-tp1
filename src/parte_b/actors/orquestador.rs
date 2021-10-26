extern crate actix;

use crate::actors::aeroservice::AeroService;
use crate::actors::hotel::Hotel;
use actix::Addr;
use std::collections::HashMap;

pub struct Orquestador {
    pub(crate) aeroservices: HashMap<String, Addr<AeroService>>,
    pub(crate) hotel: Addr<Hotel>,
}
