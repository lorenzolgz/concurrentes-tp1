extern crate actix;

use crate::actors::benchmark::Benchmark;
use crate::messages::provide_metrics::ProvideMetrics;
use actix::{Actor, Addr, Arbiter, AsyncContext, Context};
use std::time::Duration;

pub struct Cron {
    pub(crate) benchmark: Addr<Benchmark>,
}

impl Actor for Cron {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_millis(2000), |this, _ctx| {
            let arbiter: Arbiter = Arbiter::new();
            Arbiter::spawn(&arbiter, Task::execute(this.benchmark.clone()));
        });
    }
}

pub struct Task {}

impl Task {
    async fn execute(benchmark: Addr<Benchmark>) {
        benchmark.do_send(ProvideMetrics {});
    }
}
