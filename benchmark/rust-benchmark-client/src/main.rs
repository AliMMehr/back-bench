use std::sync::{Arc, Mutex};

use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let counter = Arc::new(Mutex::new(0));
    let sum_times = Arc::new(Mutex::new(Duration::new(0, 0)));

    let start = Instant::now();

    let reqwest_client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .build()?;

    let mut handles = vec![];
    for _ in 0..100 {
        let counter = counter.clone();
        let sum_times = sum_times.clone();

        let handle = tokio::spawn(send_request(reqwest_client.clone(), counter, sum_times));

        handles.push(handle)
    }

    for handle in handles {
        handle.await?
    }

    println!("Total time: {:?}", start.elapsed());

    {
        let sum_times = sum_times.lock().unwrap();
        println!("Sum times: {:?}", sum_times);
    }

    Ok(())
}

async fn send_request(
    reqwest_client: reqwest::Client,
    counter: Arc<Mutex<usize>>,
    sum_times: Arc<Mutex<Duration>>,
) {
    let saved_counter;
    {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        saved_counter = *counter;
        println!("Before: {}.", saved_counter);
    }

    let start = Instant::now();

    let resp = reqwest_client
        .get("http://localhost:3002/spin_sleep")
        .send()
        .await
        .unwrap();

    let mut resp = resp.text().await.unwrap();
    resp.truncate(10);

    let duration = start.elapsed();

    {
        let mut sum_times = sum_times.lock().unwrap();
        *sum_times += duration;
    }

    println!("After: {}: {:?}: {}", saved_counter, duration, resp);
}
