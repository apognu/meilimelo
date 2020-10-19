#[macro_use]
extern crate serde;

mod documents;
mod facets;
mod indices;
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
  documents::Update,
  facets::FacetBuilder,
  indices::Index,
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
  /// use meilimelo::prelude::*;
  ///
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
  /// ```no_run
  /// # use meilimelo::prelude::*;
  /// #
  /// # #[tokio::main]
  /// # async fn main() {
  /// let meili = MeiliMelo::new("host");
  ///
  /// for index in meili.indices().await.unwrap() {
  ///   println!("{}", index.name);
  /// }
  /// # }
  /// ```
  pub async fn indices(&'m self) -> Result<Vec<Index>, Error> {
    indices::list(self).await
  }

  /// Create a new index
  ///
  /// # Arguments
  ///
  /// * `uid` - unique ID for the new index
  /// * `name` - human-readable name for the index
  ///
  /// # Examples
  ///
  /// ```
  /// # use meilimelo::prelude::*;
  /// #
  /// # #[tokio::main]
  /// # async fn main() {
  /// MeiliMelo::new("host")
  ///   .create_index("employees", "Employees")
  ///   .await;
  /// # }
  /// ```
  pub async fn create_index<'a>(&'m self, uid: &str, name: &str) -> Result<Index, Error> {
    indices::create(self, uid, name).await
  }

  /// Delete an existing index
  ///
  /// # Arguments
  ///
  /// * `uid` - unique ID to the index to be deleted
  ///
  /// # Examples
  ///
  /// ```
  /// # use meilimelo::prelude::*;
  /// #
  /// # #[tokio::main]
  /// # async fn main() {
  /// MeiliMelo::new("host")
  ///   .delete_index("employees")
  ///   .await;
  /// # }
  /// ```
  pub async fn delete_index(&'m self, uid: &str) -> Result<(), Error> {
    indices::delete(self, uid).await
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
  /// # use meilimelo::prelude::*;
  /// #
  /// # #[derive(serde::Serialize)]
  /// # struct Employee { firstname: String, lastname: String }
  /// #
  /// let docs = vec![
  ///   Employee { firstname: "Luke".to_string(), lastname: "Skywalker".to_string() }
  /// ];
  ///
  /// MeiliMelo::new("host")
  ///   .insert("employees", &docs);
  /// ```
  pub async fn insert<T>(&'m self, index: &str, documents: &Vec<T>) -> Result<Update, Error>
  where
    T: Serialize,
  {
    documents::insert(self, index, documents).await
  }

  /// List documents in order
  ///
  /// # Arguments
  ///
  /// * `index` - name of the index to browse
  /// * `limit` - number of documents to return
  /// * `offset` - offset to the first document to return
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use meilimelo::prelude::*;
  /// #
  /// # #[derive(serde::Deserialize)]
  /// # struct Employee { firstname: String, lastname: String };
  /// #
  /// # #[tokio::main]
  /// # async fn main() {
  /// let meili = MeiliMelo::new("host");
  ///
  /// for document in &meili.list_documents::<Employee>("employees", 10, 0).await.unwrap() {
  ///   println!("{} {}", document.firstname, document.lastname);
  /// }
  /// # }
  /// ```
  pub async fn list_documents<R>(&'m self, index: &str, limit: i64, offset: i64) -> Result<Vec<R>, Error>
  where
    for<'de> R: Deserialize<'de>,
  {
    documents::list(self, index, limit, offset).await
  }

  /// List documents in order
  ///
  /// # Arguments
  ///
  /// * `index` - name of the index to browse
  /// * `uid` - Unique ID of the document to return
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use meilimelo::{prelude::*, Error};
  /// #
  /// # #[derive(serde::Deserialize)]
  /// # struct Employee;
  /// #
  /// # #[tokio::main]
  /// # async fn main() {
  /// MeiliMelo::new("")
  ///   .get_document::<Employee>("employees", "lskywalker")
  ///   .await;
  /// # }
  /// ```
  pub async fn get_document<R>(&'m self, index: &str, uid: &str) -> Result<R, Error>
  where
    for<'de> R: Deserialize<'de>,
  {
    documents::get(self, index, uid).await
  }

  /// Delete a document
  ///
  /// # Arguments
  ///
  /// * `uid` - Unique ID of the document to delete
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use meilimelo::prelude::*;
  /// #
  /// # #[tokio::main]
  /// # async fn main() {
  /// MeiliMelo::new("host")
  ///   .delete_document("employees", "lskywalker")
  ///   .await;
  /// # }
  /// ```
  pub async fn delete_document(&'m self, index: &str, uid: &str) -> Result<Update, Error> {
    documents::delete(self, index, uid).await
  }
}
