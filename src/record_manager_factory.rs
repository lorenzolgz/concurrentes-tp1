use crate::record_manager::{Record, RecordManager};
use std::sync::Arc;
use std_semaphore::Semaphore;
use std::collections::HashMap;

pub struct RecordManagerFactory {
    airline_to_semaphore: HashMap<String, Arc<Semaphore>>
}

impl RecordManagerFactory {
    pub fn new() -> RecordManagerFactory {

        let parallel_requests_count = 5;
        let airlines = vec!["AERO_1", "AERO_2", "AERO_3"];
        let mut airline_to_semaphore = HashMap::new();

        for airline in airlines {
            airline_to_semaphore.insert(
                airline.to_string(),
                Arc::new(Semaphore::new(parallel_requests_count))
            );
        }

        RecordManagerFactory { airline_to_semaphore }
    }

    pub fn get_manager(&self, record: Arc<Record>) -> Option<RecordManager> {
        let semaphore = self.airline_to_semaphore
            .get(&*record.airline);
        if semaphore.is_some(){
            Option::Some(RecordManager::new(record, (*(semaphore.unwrap())).clone()))
        } else {
            Option::None
        }
    }

}
