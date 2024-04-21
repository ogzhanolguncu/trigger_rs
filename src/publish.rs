use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;
use uuid::Uuid;

use crate::trigger_delay::TriggerTime;
use crate::trigger_headers::TriggerHeader;

#[derive(Deserialize)]
pub struct EndpointQuery {
    endpoint: String,
}

pub async fn publish(
    query: Query<EndpointQuery>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, impl IntoResponse> {
    info!(%query.endpoint, "Starting publish with");
    let message_id = Uuid::new_v4().to_string();

    if check_validity_of_url(&query.endpoint).await.is_err() {
        let error_response = Json(json!({
            "error": "Invalid destination URL. Endpoint has to resolve to a valid address",
        }));
        info!(%query.endpoint, "Returning error to the user due to malformed or invalid endpoint");
        return Err((StatusCode::BAD_REQUEST, error_response));
    }

    let TriggerHeader {
        trigger_delay,
        content_type,
        forwarded_headers,
    } = TriggerHeader::process_headers(headers);
    let trigger_duration = TriggerTime::from_string(trigger_delay);
    let trigger_body = payload;

    /*
    Note:
    Right now, our publish only allows json as a payload. I'll extend this in the future.
    Implement scheduling logic here
    */

    Ok(Json(json!({"messageId": message_id})))
}

async fn check_validity_of_url(url: &str) -> Result<bool, Error> {
    let client: Client = Client::new();
    let resp = client.head(url).send().await?;

    Ok(resp.status().is_success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_return_ok_if_valid_url() {
        let url = "https://cookie.requestcatcher.com/";
        assert!(check_validity_of_url(url).await.unwrap())
    }

    #[tokio::test]
    async fn should_return_err_if_not_valid_url() {
        let url = "localhost:666";
        assert!(check_validity_of_url(url).await.is_err())
    }
}
