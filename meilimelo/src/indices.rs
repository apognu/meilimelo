use reqwest::Method;

use crate::{prelude::*, Error};

/// MeiliSearch index descriptor
#[derive(Debug, Deserialize)]
pub struct Index {
  #[serde(rename = "primaryKey")]
  pub primary_key: Option<String>,
  pub uid: String,
  pub name: String,
  #[serde(rename = "createdAt")]
  pub created_at: Option<String>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<String>,
}

pub(crate) async fn list(meili: &MeiliMelo<'_>) -> Result<Vec<Index>, Error> {
  let response = meili
    .request(Method::GET, "/indexes")
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?
    .json::<Vec<Index>>()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(response)
}

#[derive(Debug, Serialize)]
struct IndexCreate<'a> {
  uid: &'a str,
  name: &'a str,
}

pub(crate) async fn create(meili: &MeiliMelo<'_>, uid: &str, name: &str) -> Result<Index, Error> {
  let body = IndexCreate { uid, name };

  let response = meili
    .request(Method::POST, "/indexes")
    .json(&body)
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?
    .json::<Index>()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(response)
}

pub(crate) async fn delete(meili: &MeiliMelo<'_>, uid: &str) -> Result<(), Error> {
  meili
    .request(Method::DELETE, &format!("/indexes/{}", uid))
    .send()
    .await
    .map_err(|err| Error::UpstreamError(err))?;

  Ok(())
}
