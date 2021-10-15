use crate::record_manager::{Record, RecordManager};
use std::sync::Arc;
use std_semaphore::Semaphore;
use std::collections::HashMap;

pub struct RecordManagerFactory {
    airline_to_semaphore: HashMap<String, Arc<Semaphore>>,
    package_semaphore: Arc<Semaphore>,
}

impl RecordManagerFactory {
    pub fn new() -> RecordManagerFactory {

        let mut airline_to_semaphore = HashMap::new();

        airline_to_semaphore.insert(
            "AERO_1".to_string(),
            Arc::new(Semaphore::new(5))
        );

        airline_to_semaphore.insert(
            "AERO_2".to_string(),
            Arc::new(Semaphore::new(3))
        );

        airline_to_semaphore.insert(
            "AERO_3".to_string(),
            Arc::new(Semaphore::new(1))
        );

        let package_semaphore = Arc::new(Semaphore::new(3));

        RecordManagerFactory { airline_to_semaphore, package_semaphore }
    }

    pub fn get_manager(&self, record: Arc<Record>) -> Option<RecordManager> {
        let semaphore = self.airline_to_semaphore
            .get(&*record.airline);
        if semaphore.is_some(){
            Option::Some(RecordManager::new(record, (*(semaphore.unwrap())).clone(), self.package_semaphore.clone()))
        } else {
            Option::None
        }
    }
}
