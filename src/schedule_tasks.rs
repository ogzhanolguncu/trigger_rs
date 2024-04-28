use anyhow::{Context, Result};
use axum::http::HeaderMap;
use chrono::Duration;
use reqwest::{Body, Client, Method, Response};
use serde_json::Value;

use tokio::{task, time};

use tracing::info;

use crate::trigger_cron::calculate_next_trigger_time_cron;

#[derive(Clone)]
pub struct Request {
    pub endpoint: String,
    pub headers: HeaderMap,
    pub body: Value,
    pub method: Method,
}

pub async fn start_scheduler(trigger_cron: String, trigger_request: Request) -> Result<()> {
    info!("Starting scheduler");

    loop {
        let cron = trigger_cron.clone();
        let request = trigger_request.clone();

        let next_tick = calculate_next_trigger_time_cron(cron)
            .context("Error calculating next trigger time")?;
        let duration = next_tick
            .to_std()
            .context("Error converting chrono duration to std duration")?;

        info!("Sleeping for: {:?}secs", duration.as_secs());
        time::sleep(duration).await;

        info!("Starting request, next trigger time: {:?}", next_tick);
        task::spawn(async move { start_request(request).await });
    }
}

pub async fn start_delayed_task(delay: Duration, trigger_request: Request) -> Result<Response> {
    let std_duration = delay
        .to_std()
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    time::sleep(std_duration).await;

    return start_request(trigger_request).await;
}

pub async fn start_request(trigger_request: Request) -> Result<Response> {
    let client = Client::new();

    let trigger_body_string = serde_json::to_string(&trigger_request.body).unwrap();
    let trigger_body = Body::from(trigger_body_string);

    Ok(client
        .request(trigger_request.method, trigger_request.endpoint)
        .headers(trigger_request.headers)
        .body(trigger_body)
        .send()
        .await?)
}
