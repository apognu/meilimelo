#[macro_use]
extern crate serde;

mod facets;
mod indices;
mod insert;
mod results;
mod search;

/// Most user-facing facilities can be imported through this
pub mod prelude {
  pub use crate::{
    facets::FacetBuilder,
    results::Results,
    search::{Crop, Query},
    MeiliMelo,
  };
}

use reqwest::{Client, Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::search::QueryError;

pub use self::{
  facets::FacetBuilder,
  indices::Index,
  insert::Update,
  search::{Crop, Query},
};
pub use meilimelo_macros::schema;

/// Pseudo-marker trait for MeiliSearch schemas
pub trait Schema: Default + Serialize + for<'de> Deserialize<'de> {}

/// Descriptor to a MeiliSearch instance
#[derive(Debug, Default)]
pub struct MeiliMelo<'m> {
  /// Base hostname and port to the instance, including the scheme
  host: &'m str,
  /// Secret key to be used with the requests to MeiliSearch
  secret_key: Option<&'m str>,
}

/// Errors emitted by the library
#[derive(Debug, Error)]
pub enum Error {
  /// Error originating from the communication with the instance, either upstream or when parsing its responses
  #[error("upstream error")]
  UpstreamError(#[from] reqwest::Error),
  /// The crafted query was refused by the instance
  #[error("meilisearch query error")]
  InvalidQuery(QueryError),
}

impl<'m> MeiliMelo<'m> {
  /// Creates a new descriptor to a MeiliSearch instance
  ///
  /// # Arguments
  ///
  /// * `host` - Scheme, hostname and port to the MeiliSearch instance
  pub fn new(host: &str) -> MeiliMelo {
    MeiliMelo {
      host,
      ..Default::default()
    }
  }

  pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
    let url = format!("{}{}", self.host, path);

    match self.secret_key {
      Some(key) => Client::new().request(method, &url).header("X-Meili-API-Key", key),
      None => Client::new().request(method, &url),
    }
  }

  /// Adds the secret key to be used to authenticate against MeiliSearch
  ///
  /// # Arguments
  ///
  /// * `key` - The string representation of the secret key
  ///
  /// # Examples
  ///
  /// ```
  /// let m = MeiliMelo::new("https://meilisearch.example.com:7700")
  ///   .with_secret_key("abcdef");
  /// ```
  pub fn with_secret_key(mut self, key: &'m str) -> MeiliMelo<'m> {
    self.secret_key = Some(key);
    self
  }

  /// Initialize a search query
  ///
  /// The returned struct implements the builder pattern and allows to
  /// construct the query incrementally. Please see
  /// [`Query`](search/struct.Query.html) for details on the available methods.
  ///
  /// # Arguments
  ///
  /// * `index` - The name of the index to search
  pub fn search(&'m self, index: &'m str) -> Query<'_> {
    Query::new(self, index)
  }

  /// List all available indices
  ///
  /// # Examples
  ///
  /// ```
  /// for index in m.indices().await? {
  ///   println!("{}", index.name);
  /// }
  /// ```
  pub async fn indices(&'m self) -> Result<Vec<Index>, Error> {
    indices::get_indices(self).await
  }

  /// Index a collection of documents into MeiliSearch
  ///
  /// # Arguments
  ///
  /// * index - Name of the index into which documents are to be inserted
  /// * documents - Collection of `Serialize`-able structs to insert
  ///
  /// # Examples
  ///
  /// ```
  /// let docs = vec![Employee::new("Luke", "Skywalker"), Employee::new("Leia", "Organa")];
  ///
  /// m.insert("employees", &docs);
  /// ```
  pub async fn insert<T>(&'m self, index: &str, documents: &Vec<T>) -> Result<Update, Error>
  where
    T: Serialize,
  {
    insert::insert(self, index, documents).await
  }
}
