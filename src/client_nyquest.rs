//! Asynchronous [`AsyncTelegramApi`] implementation with [`nyquest`].

use std::path::PathBuf;

use frankenstein::{AsyncTelegramApi, BASE_API_URL};
use nyquest::{AsyncClient, Body, ClientBuilder, Request};

/// Asynchronous [`AsyncTelegramApi`] implementation with [`reqwest`]
#[derive(Debug, Clone)]
pub struct Bot {
    // /// The base URL of the API.
    // pub api_url: String,
    /// The nyquest client.
    client: AsyncClient,
}

/// Possible errors when using the nyquest bot.
#[derive(thiserror::Error, Debug)]
pub enum NyquestBotError {
    /// Error from nyquest client.
    #[error("Nyquest client error: {0}")]
    NyquestError(#[from] nyquest::Error),
    /// Error encoding or decoding JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// API returned an error.
    #[error("API error: {0}")]
    ApiError(String),
}

impl Bot {
    /// Create a new [`Bot`]. You can use [`Bot::new_url`] for more options.
    pub async fn new(api_key: &str) -> Result<Self, nyquest::Error> {
        Self::new_url(format!("{BASE_API_URL}{api_key}/")).await
    }

    /// Create a new [`Bot`] with a custom API URL.
    pub async fn new_url<S: Into<String>>(api_url: S) -> Result<Self, nyquest::Error> {
        let client = ClientBuilder::default()
            .base_url(api_url.into())
            .build_async()
            .await?;
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl AsyncTelegramApi for Bot {
    type Error = NyquestBotError;

    async fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        let prepared_request = Request::post(method).with_body(Body::json(&params)?);
        let response = self.client.request(prepared_request).await?;
        let success = response.status().is_successful();
        let message = response.text().await?;
        if success {
            Ok(serde_json::from_str(&message)?)
        } else {
            Err(NyquestBotError::ApiError(message))
        }
    }

    async fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        let mut request = Request::post(method).with_body(Body::multipart(&params, files)?);
        let response = self.client.request(request).await?;
        let success = response.status().is_successful();
        let message = response.text().await?;
        if success {
            Ok(serde_json::from_str(&message)?)
        } else {
            Err(NyquestBotError::ApiError(message))
        }
    }
}
