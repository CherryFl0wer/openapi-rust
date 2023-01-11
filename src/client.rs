use reqwest::{Client as ReqwestClient, Response, StatusCode};
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use crate::errors::{Errors, Result};
use crate::settings::APISettings;


pub struct OpenAPIClient {
    client: ReqwestClient,
    settings: APISettings
}

impl OpenAPIClient {
    pub fn new(settings: APISettings) -> Self {
        let client = reqwest::ClientBuilder::new().
            timeout(settings.timeout_request).
            build().
            unwrap();

        Self {
            client,
            settings
        }
    }

    pub async fn post<P : Into<String>, D, S>(&self, path: P, body: S) -> Result<D>
        where
            D : serde::de::DeserializeOwned,
            S : serde::Serialize
    {
        let url = format!("{}{}", self.settings.host.as_str(), path.into());

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let secret_key = self.settings.secret_key.as_str();
        let response = self.client.
            post(url).
            headers(headers).
            json(&body).
            bearer_auth(secret_key).
            send().await?;
        self.handle_response(response).await
    }

    async fn handle_response<D : serde::de::DeserializeOwned>(&self, response: Response)
        -> Result<D> {
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Errors::Message("Something went wrong".into()))
        }
    }
}