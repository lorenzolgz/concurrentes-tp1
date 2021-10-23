use crate::record::Record;
use crate::record_manager::RecordManager;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std_semaphore::Semaphore;

pub struct RecordManagerFactory {
    airline_to_semaphore: HashMap<String, Arc<Semaphore>>,
    package_semaphore: Arc<Semaphore>,
    times: Arc<RwLock<Vec<u128>>>,
    log_send: Sender<String>,
}

impl RecordManagerFactory {
    pub fn new(max_requests: isize, log_send: Sender<String>) -> RecordManagerFactory {
        let mut airline_to_semaphore = HashMap::new();
        let airlines = vec!["AERO_1", "AERO_2", "AERO_3"];
        for airline in airlines {
            airline_to_semaphore
                .insert(airline.to_string(), Arc::new(Semaphore::new(max_requests)));
        }

        let package_semaphore = Arc::new(Semaphore::new(3));

        let times = Arc::new(RwLock::new(Vec::new()));

        RecordManagerFactory {
            airline_to_semaphore,
            package_semaphore,
            times,
            log_send,
        }
    }

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
