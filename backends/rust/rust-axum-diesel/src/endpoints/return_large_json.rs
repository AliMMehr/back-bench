use axum::{http::StatusCode, Json};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LargeJson {
    names: Vec<String>,
    numbers: Vec<i64>,
}

impl LargeJson {
    pub fn new() -> Self {
        let mut names = vec![];
        let mut numbers = vec![];

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let name = Alphanumeric.sample_string(&mut rng, 16);
            names.push(name);

            let num = rng.gen();
            numbers.push(num);
        }

        LargeJson { names, numbers }
    }
}

pub async fn large_json_endpoint() -> Result<Json<LargeJson>, StatusCode> {
    return Ok(LargeJson::new().into());
}
