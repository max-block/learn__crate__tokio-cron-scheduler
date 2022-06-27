use std::time::Duration;

use tokio::time;
use tokio_cron_scheduler::{Job, JobScheduler};

async fn job1() {
    println!("job1: start");
    time::sleep(Duration::from_secs(2)).await;
    println!("job1: done");
}

#[tokio::main]
async fn main() {
    let sched = JobScheduler::new().unwrap();
    sched
        .add(
            Job::new_repeated_async(Duration::from_secs(1), |_u, _l| {
                Box::pin(async { job1().await })
            })
            .unwrap(),
        )
        .unwrap();

    sched.start().unwrap();
    time::sleep(Duration::from_secs(5)).await;
}
