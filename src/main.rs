mod record_manager;
mod record_manager_factory;
mod logger;

use std::sync::Arc;
use std::{fs, thread};
use crate::record_manager::{Record, RecordManager};
use crate::record_manager_factory::RecordManagerFactory;
use crate::logger::{log_info, log_stop, log_start};
use std::collections::HashMap;

fn main() -> Result<(), csv::Error> {
    let mut routs: HashMap<String, i32> = HashMap::new();
    let (logger_handle, log_send) = log_start();
    let mut reservations = vec![];

    let csv = fs::read_to_string("./src/reservations.csv")
        .expect("Something went wrong reading the file");

    println!("[Main] reservations.csv: \n {}", csv);
    log_info(format!("[Main] reservations.csv: \n {}", csv), log_send.clone());

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let manager_factory = RecordManagerFactory::new();

    for record in reader.deserialize() {
        let record: Record = record?;

        let rout = format!("{}-{}", record.origin, record.destination);
        let local_rout = rout.clone();

        match routs.get_mut(&*rout) {
            Some(routs) => {
                *routs += 1;
            },
            None => {
                routs.insert(rout, 1);
            }
        }

        println!("[Main] {}: {}", format!("{}-{}", record.origin, record.destination), routs[&local_rout]);

        let airline: String = record.airline.to_string();
        let optional_manager: Option<RecordManager> = (&manager_factory).get_manager(Arc::from(record));
        if optional_manager.is_some(){
            reservations.push(thread::spawn( move ||
                optional_manager
                    .unwrap()
                    .trigger_requests_until_success()
            ));
        } else {
            println!("Unable to find aero semaphore for {}", airline)
        }
    }

    for reservation in reservations {
        reservation.join().expect("Unable to join on the associated thread");
    }

    log_stop(log_send.clone(), logger_handle);

    Ok(())
}
