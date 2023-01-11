use std::time::Duration;
use crate::requests::request_completion::*;

#[derive(Debug)]
pub struct APISettings {
    /// API secret key used for every call to identify
    pub secret_key: String,
    /// Organization identifier
    pub organization_id: Option<String>,
    /// By default 10.0 seconds
    pub timeout_request: Duration,
    /// Base request
    pub host: String
}

impl Default for APISettings {
    fn default() -> Self {
        Self {
            secret_key: "".into(),
            organization_id: None,
            timeout_request: Duration::from_secs_f32(10.0),
            host: "https://api.openai.com/v1".into(),
        }
    }
}



impl APISettings {
    /// Create a new OpenAPI execution context
    pub fn new(secret_key: String, organization_id: Option<String>) -> Self {
        Self {
            secret_key,
            organization_id,
            ..APISettings::default()
        }
    }

    /// Create a new OpenAPI execution context based on env
    pub fn new_with_env() -> Self {
        let secret_key = std::env::var("OPENAPI_SECRET_KEY").ok().unwrap();
        let organization_id = std::env::var("OPENAPI_ORGANIZATION_ID").ok();
        Self {
            secret_key,
            organization_id,
            ..APISettings::default()
        }
    }

    pub fn set_organization_id<T : Into<String>>(mut self, org_id : T) -> Self {
        self.organization_id = Some(org_id.into());
        self
    }

    pub fn set_timeout_request(mut self, duration : Duration) -> Self {
        self.timeout_request = duration;
        self
    }

    pub fn set_host<T : Into<String>>(mut self, host : T) -> Self {
        self.host = host.into();
        self
    }
}