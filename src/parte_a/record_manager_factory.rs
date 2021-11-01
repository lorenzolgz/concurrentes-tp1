extern crate common;

use crate::common::airlines::AIRLINES;
use crate::common::record::Record;
use crate::record_manager::RecordManager;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std_semaphore::Semaphore;

/// A struct that contains references to all semaphores of each airline and a vector(whit a RwLock)
/// of the time taken to make a reservation.
pub struct RecordManagerFactory {
    airline_to_semaphore: HashMap<String, Arc<Semaphore>>,
    package_semaphore: Arc<Semaphore>,
    times: Arc<RwLock<Vec<u128>>>,
    log_send: Sender<String>,
}

impl RecordManagerFactory {
    /// Inicializa el RecordManagerFactory con semáforos de tamaño max_requests para las aerolíneas. Además recibe un canal de Sender para loguear su funcionamiento.
    pub fn new(max_requests: isize, log_send: Sender<String>) -> RecordManagerFactory {
        let mut airline_to_semaphore = HashMap::new();
        for airline in AIRLINES {
            airline_to_semaphore
                .insert(airline.to_string(), Arc::new(Semaphore::new(max_requests)));
        }

        let package_semaphore = Arc::new(Semaphore::new(max_requests));

        let times = Arc::new(RwLock::new(Vec::new()));

        RecordManagerFactory {
            airline_to_semaphore,
            package_semaphore,
            times,
            log_send,
        }
    }

    /// Receives a record and returns an Option that in case of founding the airline semaphore
    /// returns a RecordManager for that Record, otherwise it returns an Option::None.
    pub fn get_manager(&self, record: Arc<Record>) -> Option<RecordManager> {
        self.airline_to_semaphore.get(&*record.airline).map_or_else(
            || Option::None,
            |sem| {
                Option::Some(RecordManager::new(
                    record,
                    (*sem).clone(),
                    self.package_semaphore.clone(),
                    self.times.clone(),
                    self.log_send.clone(),
                ))
            },
        )
    }
}
