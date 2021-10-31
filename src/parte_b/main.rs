extern crate actix;
mod actors;
mod messages;

use crate::actors::aero_service::AeroService;
use crate::actors::benchmark::Benchmark;
use crate::actors::cron::Cron;
use crate::actors::hotel_service::HotelService;
use crate::actors::logger::Logger;
use crate::actors::orchestrator::Orchestrator;
use crate::messages::entry::Entry;
use actix::{Actor, SyncArbiter, System};
use common::airlines::AIRLINES;
use common::helper::{
    get_csv_file_path, get_log_file_name, get_log_output_path, get_max_requests_count,
};
use common::record::Record;
use common::routs_stats::RoutsStats;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::sync::Arc;
use std::time::SystemTime;

fn main() {
    let file_name = get_log_file_name(get_log_output_path("parte_b".to_string()));
    let file_logger = File::create(file_name).expect("Error creating logger file");
    let max_requests = get_max_requests_count() as usize;
    let path = get_csv_file_path();
    let csv = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let system = System::new();

    system.block_on(async {
        let logger = Logger { file: file_logger }.start();
        let mut aeroservices = HashMap::new();
        let ref_logger = Arc::from(logger.clone());
        for airline in AIRLINES {
            let copy_logger = ref_logger.clone();
            aeroservices.insert(
                airline.to_string(),
                SyncArbiter::start(max_requests, move || AeroService {
                    id: airline.to_string(),
                    logger: copy_logger.clone(),
                }),
            );
        }

        let hotel_service = SyncArbiter::start(max_requests, move || HotelService {
            logger: ref_logger.clone(),
        });
        let benchmark_service = Benchmark {
            finished_requests: 0,
            average_time: 0.0,
            already_provided: true,
            stats: RoutsStats::new(),
            logger: logger.clone(),
        }
        .start();
        Cron {
            benchmark: benchmark_service.clone(),
        }
        .start();
        let otro_orq = Orchestrator {
            aeroservices,
            hotel: hotel_service,
            benchmark: benchmark_service,
            logger,
        }
        .start();
        for record in reader.deserialize() {
            let record: Record = record.expect("Unable to parse record");
            otro_orq.do_send(Entry {
                aero_id: record.airline.to_string(),
                origin: record.origin.to_string(),
                destination: record.destination.to_string(),
                includes_hotel: record.package,
                sender: otro_orq.clone(),
                start_time: SystemTime::now(),
            });
        }
    });

    system.run().expect("Unable to start event loop from main");
}
