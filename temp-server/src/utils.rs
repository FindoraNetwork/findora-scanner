use poem::http::StatusCode;
use reqwest::Result;
use serde::{de::DeserializeOwned, Serialize};

pub async fn request_get<T>(url: String) -> Result<T>
where
    T: Serialize + DeserializeOwned,
{
    reqwest::get(url).await?.json::<T>().await
}

pub fn err_handle(e: reqwest::Error) -> poem::Error {
    poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
}
