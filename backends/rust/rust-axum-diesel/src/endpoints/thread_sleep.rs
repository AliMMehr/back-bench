use std::time::Duration;

pub async fn thread_sleep() -> &'static str {
    tokio::time::sleep(Duration::from_secs(10)).await;
    return "Thread sleep!";
}
