use reqwest::Method;
use serde::{Deserialize, Serialize};

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

pub(crate) async fn list<R>(meili: &MeiliMelo<'_>, index: &str, limit: i64, offset: i64) -> Result<Vec<R>, Error>
where
  for<'de> R: Deserialize<'de>,
{
  let response = meili
    .request(
      Method::GET,
      &format!("/indexes/{}/documents?limit={}&offset={}", index, limit, offset),
    )
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?
    .json::<Vec<R>>()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(response)
}

pub(crate) async fn get<R>(meili: &MeiliMelo<'_>, index: &str, uid: &str) -> Result<R, Error>
where
  for<'de> R: Deserialize<'de>,
{
  let response = meili
    .request(Method::GET, &format!("/indexes/{}/documents/{}", index, uid))
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?
    .json::<R>()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(response)
}

pub(crate) async fn delete(meili: &MeiliMelo<'_>, index: &str, uid: &str) -> Result<Update, Error> {
  let response = meili
    .request(Method::GET, &format!("/indexes/{}/documents/{}", index, uid))
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?
    .json::<Update>()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(response)
}
