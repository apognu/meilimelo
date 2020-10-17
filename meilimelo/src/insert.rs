use reqwest::Method;
use serde::Serialize;

use crate::{prelude::*, Error};

/// Descriptor for an asynchronous upstream operation
#[derive(Debug, Deserialize)]
pub struct Update {
  #[serde(rename = "updateId")]
  pub id: i64,
}

pub(crate) async fn insert<T>(meili: &MeiliMelo<'_>, index: &str, documents: &Vec<T>) -> Result<Update, Error>
where
  T: Serialize,
{
  let response = meili
    .request(Method::POST, &format!("/indexes/{}/documents", index))
    .json(&documents)
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?
    .json::<Update>()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(response)
}
