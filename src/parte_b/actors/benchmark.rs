extern crate actix;

use crate::messages::provide_metrics::ProvideMetrics;
use crate::messages::request_completed::RequestCompleted;
use actix::{Actor, Context, Handler};
use common::helper::stringify_top_10;
use common::routs_stats::RoutsStats;
use std::time::Duration;

pub struct Benchmark {
    pub(crate) finished_requests: u128,
    pub(crate) average_time: f64,
    pub(crate) stats: RoutsStats,
    pub(crate) already_provided: bool,
}

impl Actor for Benchmark {
    type Context = Context<Self>;
}

impl Handler<RequestCompleted> for Benchmark {
    type Result = ();
    fn handle(&mut self, msg: RequestCompleted, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[BENCHMARKER] recibi request completed, elapsed={}, origin={}, dest={}",
            msg.time_elapsed.map_or_else(
                || { "N/A".to_string() },
                |duration| { duration.as_millis().to_string() }
            ),
            msg.origin,
            msg.destination
        );
        msg.time_elapsed.map_or((), |duration| {
            self.update_average_time(duration);
        });
        self.stats.add(msg.origin, msg.destination);
        self.already_provided = false;
    }
}

impl Handler<ProvideMetrics> for Benchmark {
    type Result = ();
    fn handle(&mut self, _msg: ProvideMetrics, _ctx: &mut Context<Self>) -> Self::Result {
        if !self.already_provided {
            println!("[BENCHMARKER] Delivering metrics");
            println!(
                "[BENCHMARKER] Average time to completion is: {} millis",
                self.average_time
            );
            println!(
                "[BENCHMARKER] {}",
                stringify_top_10(self.stats.build_top_10())
            );
            self.already_provided = true;
        }
    }
}

impl Benchmark {
    fn update_average_time(&mut self, elapsed_time: Duration) {
        let updated_finished_count = self.finished_requests + 1;
        self.average_time = (self.average_time * (self.finished_requests as f64)
            + (elapsed_time.as_millis() as f64))
            / (updated_finished_count as f64);
        self.finished_requests = updated_finished_count;
    }
}
