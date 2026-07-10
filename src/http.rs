use reqwest::{header::HeaderMap, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::Error;

pub(crate) struct HttpTransport {
    client: reqwest::Client,
    base_url: String,
    auth_header: String,
    user_agent: String,
}

impl HttpTransport {
    pub fn new(
        client: reqwest::Client,
        base_url: String,
        auth_header: String,
        user_agent: String,
    ) -> Self {
        Self {
            client,
            base_url,
            auth_header,
            user_agent,
        }
    }

    pub async fn request_json<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let resp = self.execute(method, path, body).await?;
        let result = resp.json::<R>().await?;
        Ok(result)
    }

    pub async fn request_json_with_headers<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<(R, HeaderMap), Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let resp = self.execute(method, path, body).await?;
        let headers = resp.headers().clone();
        let result = resp.json::<R>().await?;
        Ok((result, headers))
    }

    pub async fn request_empty<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.execute(method, path, body).await?;
        Ok(())
    }

    pub(crate) async fn request_json_custom_auth<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
        auth_header: &str,
    ) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let resp = self
            .execute_with_auth(method, path, body, auth_header)
            .await?;
        let result = resp.json::<R>().await?;
        Ok(result)
    }

    async fn execute<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<reqwest::Response, Error>
    where
        T: Serialize,
    {
        self.execute_with_auth(method, path, body, &self.auth_header)
            .await
    }

    async fn execute_with_auth<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
        auth_header: &str,
    ) -> Result<reqwest::Response, Error>
    where
        T: Serialize,
    {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);

        let mut req = self
            .client
            .request(method, &url)
            .header("Authorization", auth_header)
            .header("User-Agent", &self.user_agent);

        if let Some(b) = body {
            req = req.json(b);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Self::map_error(status, &body_text));
        }

        Ok(resp)
    }

    fn map_error(status: StatusCode, body: &str) -> Error {
        match status.as_u16() {
            400 => Error::BadRequest(body.to_owned()),
            401 => Error::Unauthorized(body.to_owned()),
            403 => Error::Forbidden(body.to_owned()),
            404 => Error::NotFound(body.to_owned()),
            409 => Error::Conflict(body.to_owned()),
            422 => Error::UnprocessableEntity(body.to_owned()),
            429 => Error::TooManyRequests(body.to_owned()),
            _ if status.is_server_error() => Error::Server(format!(
                "unexpected status code {} with body: {}",
                status.as_u16(),
                body
            )),
            _ => Error::Client(format!(
                "unexpected status code {} with body: {}",
                status.as_u16(),
                body
            )),
        }
    }
}
