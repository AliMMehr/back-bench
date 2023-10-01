use std::{
    ops::Add,
    time::{Duration, Instant},
};

pub async fn spin_sleep() -> &'static str {
    let end_time = Instant::now().add(Duration::from_secs(10));
    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;
        if Instant::now() > end_time {
            break;
        }
    }

    return "Spin sleep!";
}
