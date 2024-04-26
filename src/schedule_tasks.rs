use axum::http::{request, HeaderMap};
use chrono::{Duration, TimeDelta};
use reqwest::{Body, Client, Error, Method, Response};
use serde_json::Value;
use std::sync::Arc;
use tokio::{task, time};
use tracing::warn;

use crate::{trigger_cron::calculate_next_trigger_time_cron, trigger_headers};

#[derive(Clone)]
pub struct Request {
    pub endpoint: String,
    pub headers: HeaderMap,
    pub body: Value,
    pub method: Method,
}

pub async fn start_scheduler(trigger_cron: String, trigger_request: Request) -> Result<(), Error> {
    println!("Starting scheduler");

    // let trigger_clone = Arc::new(trigger_cron);
    loop {
        let cron = trigger_cron.to_owned();
        let request = trigger_request.clone();
        if let Ok(next) = calculate_next_trigger_time_cron(cron) {
            if let Ok(duration) = next.to_std() {
                time::sleep(duration).await;

                task::spawn(async { start_request(request) });
            } else {
                warn!("Error converting chrono duration to std duration");
            }
        } else {
            warn!("Error calculating next trigger time");
        }
    }
}

pub async fn start_delayed_task(delay: Duration, trigger_request: Request) {
    let std_duration = delay
        .to_std()
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    time::sleep(std_duration).await;
    start_request(trigger_request).await;
}

pub async fn start_request(trigger_request: Request) -> Result<Response, Error> {
    let client = Client::new();

    let trigger_body_string = serde_json::to_string(&trigger_request.body).unwrap();
    let trigger_body = Body::from(trigger_body_string);

    client
        .request(trigger_request.method, trigger_request.endpoint)
        .headers(trigger_request.headers)
        .body(trigger_body)
        .send()
        .await
}
