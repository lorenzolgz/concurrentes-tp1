extern crate actix;

use crate::actors::benchmark::Benchmark;
use crate::messages::provide_metrics::ProvideMetrics;
use actix::{Actor, Addr, Arbiter, AsyncContext, Context};
use std::time::Duration;

/// Struct Representing a CronJob
/// Holds a Benchmark adress as its responsibility is to send messages to it
pub struct Cron {
    pub(crate) benchmark: Addr<Benchmark>,
}

impl Actor for Cron {
    type Context = Context<Self>;
    /// Every 2000 millis a ProvideMetrics message is sent to the Benchmark to print
    /// the current metrics
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_millis(2000), |this, _ctx| {
            let arbiter: Arbiter = Arbiter::new();
            Arbiter::spawn(&arbiter, Cron::ask_from_metrics(this.benchmark.clone()));
        });
    }
}

impl Cron {
    async fn ask_from_metrics(benchmark: Addr<Benchmark>) {
        benchmark.do_send(ProvideMetrics {});
    }
}
