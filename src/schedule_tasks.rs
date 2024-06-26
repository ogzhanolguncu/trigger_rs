use anyhow::{Context, Error, Result};
use axum::http::HeaderMap;
use chrono::Duration;
use reqwest::{Body, Client, Method, Response};
use serde_json::Value;
use tokio::time::sleep;
use tokio::{task, time};

use crate::sql::{update_task, TaskStatus};
use crate::utils::format_duration;
use tracing::{error, info};

use crate::trigger_cron::calculate_next_trigger_time_cron;

#[derive(Clone)]
pub struct Request {
    pub endpoint: String,
    pub headers: HeaderMap,
    pub body: Value,
    pub method: Method,
}

pub async fn start_scheduler(
    message_id: &str,
    trigger_cron: String,
    trigger_request: Request,
) -> Result<()> {
    info!("Starting scheduler");

    let message_id = message_id.to_owned();
    let message_id_clone = message_id.clone();
    loop {
        let cron = trigger_cron.clone();
        let request = trigger_request.clone();
        let message_id = message_id.clone();

        let next_tick = calculate_next_trigger_time_cron(cron)
            .context("Error calculating next trigger time")?;
        let duration = next_tick
            .to_std()
            .context("Error converting chrono duration to std duration")?;

        update_task(message_id_clone.as_str(), TaskStatus::Pending).await;

        info!("Sleeping for: {:?}secs", duration.as_secs());
        time::sleep(duration).await;

        info!("Starting request, next trigger time: {:?}", next_tick);
        task::spawn(async move { start_request(message_id.as_str(), request).await });
    }
}

pub async fn start_delayed_task(
    message_id: &str,
    delay: Duration,
    trigger_request: Request,
) -> Result<Response> {
    update_task(message_id, TaskStatus::Pending).await;

    let std_duration = delay
        .to_std()
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    time::sleep(std_duration).await;

    start_request(message_id, trigger_request).await
}

pub async fn start_request(message_id: &str, trigger_request: Request) -> Result<Response, Error> {
    let client = Client::new();

    let mut attempts = 0;
    let max_attempts = 2;

    loop {
        let trigger_body_string = serde_json::to_string(&trigger_request.body).unwrap();
        let trigger_body = Body::from(trigger_body_string);

        let response = client
            .request(
                trigger_request.method.clone(),
                trigger_request.endpoint.clone(),
            )
            .headers(trigger_request.headers.clone())
            .body(trigger_body)
            .send()
            .await;

        match response {
            Ok(resp) => return Ok(resp),
            Err(_) if attempts < max_attempts => {
                update_task(message_id, TaskStatus::Retry).await;
                let delay_secs = f64::exp(2.5 * (attempts as f64)).min(86400.0);
                let duration = Duration::seconds(delay_secs as i64);

                info!(
                    "Error sending request, retrying in {} ",
                    format_duration(duration)
                );
                sleep(duration.to_std().unwrap()).await;

                attempts += 1;
            }
            Err(e) => {
                update_task(message_id, TaskStatus::Failed).await;
                error!("Error sending request: {:?}", e);
                return Err(anyhow::Error::from(e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn should_generate_request_correctly() {
        let endpoint = "https://faxr.requestcatcher.com/test".to_string();

        let request = Request {
            endpoint: endpoint.clone(),
            headers: HeaderMap::new(),
            body: json!({"key": "value"}),
            method: reqwest::Method::GET,
        };

        let response = start_request("1", request).await.unwrap();

        assert_eq!(response.status(), 200);

        assert_eq!(response.url().to_string(), endpoint);
    }
}
