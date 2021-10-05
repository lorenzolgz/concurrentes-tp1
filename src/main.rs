mod record_manager;

use std::sync::Arc;
use std::{fs, thread};
use std::collections::HashMap;
use std_semaphore::Semaphore;
use crate::record_manager::{Record, RecordManager};

fn main() -> Result<(), csv::Error> {
    let mut reservations = vec![];
    let mut managers = vec![];

    let csv = fs::read_to_string("./src/reservations.csv")
        .expect("Something went wrong reading the file");

    println!("[Main] reservations.csv: {}", csv);
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let airline_to_semaphore = load_airlines();

    for record in reader.deserialize() {
        let record: Record = record?;
        let sem = get_semaphore(&airline_to_semaphore, &record.airline);
        if sem.is_some() {
            managers.push(RecordManager::new(Arc::from(record), sem.unwrap()))
        } else {
            println!("Unable to find semaphore for airline {}", record.airline)
        }
    }

    for manager in managers {
        reservations.push(thread::spawn(move||manager.trigger_request()));
    }

    for reservation in reservations {
        reservation.join().expect("Unable to join on the associated thread");
    }

    Ok(())
}

fn load_airlines() -> HashMap<std::string::String, Arc<Semaphore>> {
    let mut airline_to_semaphore = HashMap::new();
    let parallel_requests_count = 5;
    airline_to_semaphore.insert(
        "AERO_1".to_string(),
        Arc::new(Semaphore::new(parallel_requests_count))
    );
    airline_to_semaphore.insert(
        "AERO_2".to_string(),
        Arc::new(Semaphore::new(parallel_requests_count))
    );
    airline_to_semaphore.insert(
        "AERO_3".to_string(),
        Arc::new(Semaphore::new(parallel_requests_count))
    );
    airline_to_semaphore
}

fn get_semaphore(airline_to_semaphore: &HashMap<String, Arc<Semaphore>>, airline: &String) -> Option<Arc<Semaphore>> {
    let semaphore = airline_to_semaphore.get(airline);
    if semaphore.is_some() {
        Option::Some(semaphore.unwrap().clone())
    } else {
        Option::None
    }
}