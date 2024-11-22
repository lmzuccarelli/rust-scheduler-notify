use clokwerk::{AsyncScheduler, Job, TimeUnits};
use notify_rust::{Notification, Timeout};
// Import week days and WeekDay
//use clokwerk::Interval::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Create a new scheduler
    let mut scheduler = AsyncScheduler::new();
    // Add some tasks to it
    scheduler
        .every(1.minutes())
        .plus(30.seconds())
        .run(|| async {
            notification().await;
        });

    //loop {
    //    scheduler.run_pending().await;
    //    tokio::time::sleep(Duration::from_millis(10)).await;
    //}

    // Or spawn a task to run it forever

    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    loop {}
}

async fn notification() {
    Notification::new()
        .summary("Notification")
        .body("hey baba wakey wakey.")
        .icon("firefox")
        .timeout(Timeout::Milliseconds(6000))
        .show()
        .unwrap();
}
