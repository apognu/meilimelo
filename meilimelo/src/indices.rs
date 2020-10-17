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

pub(crate) async fn get_indices(meili: &MeiliMelo<'_>) -> Result<Vec<Index>, Error> {
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
