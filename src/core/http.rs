//! HTTP client integration for MachTUI.
//! Provides high-level async data fetching utilities.

use crate::talon::Cmd;
use serde::de::DeserializeOwned;

pub struct HttpClient;

impl HttpClient {
    /// Fetches JSON from a URL and returns a Talon Command that dispatches a message.
    pub fn fetch_json<T, M, F>(url: &str, mapper: F) -> Cmd<M>
    where
        T: DeserializeOwned + Send + 'static,
        M: Send + 'static,
        F: Fn(Result<T, String>) -> M + Send + Sync + 'static,
    {
        let url = url.to_string();
        Box::pin(async move {
            let client = reqwest::Client::new();
            let res = client.get(url).send().await;

            match res {
                Ok(resp) => {
                    let json = resp.json::<T>().await;
                    match json {
                        Ok(data) => Some(mapper(Ok(data))),
                        Err(e) => Some(mapper(Err(e.to_string()))),
                    }
                }
                Err(e) => Some(mapper(Err(e.to_string()))),
            }
        })
    }
}
