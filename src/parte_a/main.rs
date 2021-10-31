mod logger;
mod record_manager;
mod record_manager_factory;

extern crate common;

use crate::common::helper::get_max_requests_count;
use crate::common::record::Record;
use crate::logger::{log_info, log_start, log_stop};
use crate::record_manager::RecordManager;
use crate::record_manager_factory::RecordManagerFactory;
use common::helper::{get_csv_file_path, stringify_top_10};
use common::routs_stats::RoutsStats;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

fn main() -> Result<(), csv::Error> {
    let rout_stats = Arc::new(Mutex::new(RoutsStats::new()));

    let (logger_handle, log_send) = log_start();

    let mut reservations = vec![];
    let max_requests = get_max_requests_count();
    log_info(
        format!(
            "[Main] Starting process with {} parallel requests at most per server",
            max_requests
        ),
        log_send.clone(),
    );

    let path = get_csv_file_path();
    let csv = fs::read_to_string(path).expect("Something went wrong reading the file");

    log_info(
        format!("[Main] reservations.csv: \n{}", csv),
        log_send.clone(),
    );

    let clone_rout_stats = rout_stats.clone();
    let stats_log_send = log_send.clone();
    let stats_handle = thread::spawn(move || rout_stats_monitor(clone_rout_stats, stats_log_send));

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let manager_factory = RecordManagerFactory::new(max_requests, log_send.clone());

    for record in reader.deserialize() {
        let record: Record = record?;

        let local_rout_stats = rout_stats.clone();

        match local_rout_stats.lock() {
            Ok(mut g) => g.add(record.origin.to_string(), record.destination.to_string()),
            Err(e) => panic!("Mutex error at rout_stats {}", e),
        }

        let airline: String = record.airline.to_string();
        let optional_manager: Option<RecordManager> =
            (&manager_factory).get_manager(Arc::from(record));
        optional_manager.map_or_else(
            || {
                log_info(
                    format!("[Main] Unable to find service semaphore for {}", airline),
                    log_send.clone(),
                )
            },
            |manager| {
                reservations.push(thread::spawn(move || {
                    manager.trigger_requests_until_success()
                }))
            },
        )
    }

    for reservation in reservations {
        reservation
            .join()
            .expect("Unable to join on the associated thread");
    }

    match rout_stats.lock() {
        Ok(mut g) => g.stop(),
        Err(e) => panic!("Mutex error at rout_stats {}", e),
    }
    stats_handle.join().expect("Unable to join stats_handle");

    log_stop(log_send, logger_handle);

    Ok(())
}

/// Every 10 seconds this function prints the top 10 most requested routs
fn rout_stats_monitor(clone_rout_stats: Arc<Mutex<RoutsStats>>, log: Sender<String>) {
    loop {
        thread::sleep(Duration::from_secs(10));

        let mut guard = match clone_rout_stats.lock() {
            Ok(g) => g,
            Err(e) => panic!("Mutex error of rout_stats at rout_stats_monitor {}", e),
        };

        log_info(stringify_top_10(guard.build_top_10()), log.clone());

        if guard.stopped {
            break;
        }
    }
}
