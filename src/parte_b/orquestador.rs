extern crate actix;

use actix::{
    Addr,
};
use crate::aeroservice::AeroService;
use crate::hotel::Hotel;
use std::collections::HashMap;

pub struct Orquestador {
    pub(crate) aeroservices: HashMap<usize, Addr<AeroService>>,
    pub(crate) hotel: Addr<Hotel>,
}