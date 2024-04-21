use std::str::FromStr;

use axum::http::{HeaderMap, HeaderName, HeaderValue};

const TRIGGER_PREFIX: &str = "trigger-forward-";

#[derive(Debug)]
pub struct TriggerHeader {
    pub trigger_delay: Option<String>,
    pub content_type: Option<String>,
    pub forwarded_headers: HeaderMap,
}

impl TriggerHeader {
    pub fn process_headers(headers: HeaderMap) -> Self {
        let mut parsed_headers = Self {
            trigger_delay: None,
            content_type: None,
            forwarded_headers: HeaderMap::new(),
        };

        for (name, value) in headers.iter() {
            let value_str = match value.to_str() {
                Ok(v) => v.to_string(),
                Err(_) => continue,
            };

            match name.as_str() {
                "content-type" => parsed_headers.content_type = Some(value_str),
                "trigger-delay" => parsed_headers.trigger_delay = Some(value_str),
                name if name.starts_with(TRIGGER_PREFIX) => {
                    TriggerHeader::capture_forward_headers(&mut parsed_headers, name, value_str)
                }
                _ => {}
            }
        }

        parsed_headers
    }

    fn capture_forward_headers(
        parsed_headers: &mut TriggerHeader,
        forwarded_header_name: &str,
        forwarded_header_value: String,
    ) {
        let header_name = forwarded_header_name
            .trim_start_matches(TRIGGER_PREFIX)
            .to_string();

        if let Ok(header_name) = HeaderName::from_str(&header_name) {
            if let Ok(header_value) = HeaderValue::from_str(forwarded_header_value.as_str()) {
                parsed_headers
                    .forwarded_headers
                    .insert(header_name, header_value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_trigger_delay() {
        let mut header_map = HeaderMap::new();
        header_map.insert("trigger-delay", "5m".parse().unwrap());
        assert_eq!(
            TriggerHeader::process_headers(header_map).trigger_delay,
            Some("5m".to_string())
        )
    }

    #[test]
    fn should_return_none_when_trigger_delay_is_missing() {
        let header_map = HeaderMap::new();
        assert_eq!(
            TriggerHeader::process_headers(header_map).trigger_delay,
            None
        )
    }

    #[test]
    fn should_return_content_type() {
        let mut header_map = HeaderMap::new();
        header_map.insert("content-type", "application/json".parse().unwrap());
        assert_eq!(
            TriggerHeader::process_headers(header_map).content_type,
            Some("application/json".to_string())
        )
    }

    #[test]
    fn should_return_none_when_content_type_is_missing() {
        let header_map = HeaderMap::new();
        assert_eq!(
            TriggerHeader::process_headers(header_map).content_type,
            None
        )
    }

    #[test]
    fn should_return_forwarded_headers() {
        let mut header_map = HeaderMap::new();
        header_map.insert("trigger-forward-name", "oz".parse().unwrap());
        header_map.insert("trigger-forward-cookie", "monster".parse().unwrap());

        assert_eq!(
            TriggerHeader::process_headers(header_map)
                .forwarded_headers
                .get("name")
                .unwrap(),
            "oz"
        )
    }
}
