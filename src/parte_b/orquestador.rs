extern crate actix;

use crate::aeroservice::AeroService;
use crate::hotel::Hotel;
use actix::Addr;
use std::collections::HashMap;

pub struct Orquestador {
    pub(crate) aeroservices: HashMap<String, Addr<AeroService>>,
    pub(crate) hotel: Addr<Hotel>,
}
