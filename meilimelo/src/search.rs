use reqwest::{Method, StatusCode};
use serde::Deserialize;

use crate::{facets::Facets, results::Results, Error, MeiliMelo, Schema};

/// Utility to build a search query
///
/// This implements the builder pattern, so you can incrementally build the
/// request you want to perform. The search query can finally be run by using
/// [`Query::run()`](#method.run).
///
/// # Examples
///
/// ```
/// let results = m.search("employees")
///   .query("johnson")
///   .facets(FacetBuilder::new("company", "ACME Corp").build())
///   .distribution(&["roles"])
///   .limit(10)
///   .run::<Employee>()
///   .await;
/// ```
#[derive(Debug, Serialize)]
pub struct Query<'m> {
  #[serde(skip_serializing)]
  meili: &'m MeiliMelo<'m>,

  #[serde(skip_serializing)]
  index: &'m str,
  #[serde(rename = "q")]
  query: Option<&'m str>,
  filters: Option<&'m str>,
  #[serde(rename = "facetFilters")]
  facets: Option<Vec<Vec<String>>>,
  limit: Option<i64>,
  offset: Option<i64>,

  #[serde(rename = "attributesToRetrieve")]
  retrieve: Option<&'m [&'m str]>,
  #[serde(rename = "attributesToCrop")]
  crop: Option<Vec<String>>,
  #[serde(rename = "cropLength")]
  crop_length: Option<i64>,
  #[serde(rename = "attributesToHighlight")]
  highlight: Option<&'m [&'m str]>,
  #[serde(rename = "facetsDistribution")]
  distribution: Option<&'m [&'m str]>,
  #[serde(rename = "matches")]
  matches: bool,
}

#[derive(Debug, Deserialize)]
pub struct QueryError {
  #[serde(rename = "errorType")]
  pub kind: String,
  #[serde(rename = "errorCode")]
  pub code: String,
  pub message: String,
  #[serde(rename = "errorLink")]
  pub link: String,
}

impl<'m> Query<'m> {
  pub(crate) fn new(meili: &'m MeiliMelo, index: &'m str) -> Query<'m> {
    Query {
      meili,
      index,
      query: None,
      filters: None,
      facets: None,
      limit: None,
      offset: None,
      retrieve: None,
      crop: None,
      crop_length: None,
      highlight: None,
      distribution: None,
      matches: false,
    }
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#query-q)
  ///
  /// # Arguments
  ///
  /// * `query` - string of the query to be used
  ///
  /// # Examples
  ///
  /// ```
  /// q.query("streamer");
  /// ```
  pub fn query(mut self, query: &'m str) -> Query {
    self.query = Some(query);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#filters)
  ///
  /// # Arguments
  ///
  /// * `filters` - string representing to filter to be applied
  ///
  /// # Examples
  ///
  /// ```
  /// q.filters("company = ACME AND age > 23");
  /// ```
  pub fn filters(mut self, filters: &'m str) -> Query {
    self.filters = Some(filters);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#limit)
  ///
  /// # Arguments
  ///
  /// * `limit` - number of documents to be returned
  ///
  /// # Examples
  ///
  /// ```
  /// q.limit(10);
  /// ```
  pub fn limit(mut self, limit: i64) -> Query<'m> {
    self.limit = Some(limit);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#skip)
  ///
  /// # Arguments
  ///
  /// * `offset` - number of documents to skip
  ///
  /// # Examples
  ///
  /// ```
  /// q.offset(20);
  /// ```
  pub fn offset(mut self, offset: i64) -> Query<'m> {
    self.offset = Some(offset);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#facetFilters)
  ///
  /// [`FacetBuilder`](facets/struct.FacetBuilder.html) must be used to create the facet statement.
  ///
  /// # Arguments
  ///
  /// * `facets` - facets to apply to the search
  ///
  /// # Examples
  ///
  /// ```
  /// q.facets(FacetBuilder::new("company", "ACME Corp")
  ///   .or("company": "Big Corp")
  ///   .and("roles", "Tech")
  ///   .build());
  /// ```
  pub fn facets(mut self, facets: Facets) -> Query<'m> {
    self.facets = Some(facets.get());
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#attributesToRetrieve)
  ///
  /// # Arguments
  ///
  /// * `attributes` - slice of attributes to return
  ///
  /// # Examples
  ///
  /// ```
  /// q.retrieve(&["firstname", "lastname"]);
  /// ```
  pub fn retrieve(mut self, attributes: &'m [&'m str]) -> Query<'m> {
    self.retrieve = Some(attributes);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#facetsDistribution)
  ///
  /// # Arguments
  ///
  /// * `attributes` - slice of facets for which to return distribution statistics
  ///
  /// # Examples
  ///
  /// ```
  /// q.distribution(&["firstname", "lastname"]);
  /// ```
  pub fn distribution(mut self, facets: &'m [&'m str]) -> Query<'m> {
    self.distribution = Some(facets);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#attributesToCrop)
  ///
  /// # Arguments
  ///
  /// * `attributes` - slice of attributes to crop to [`crop_length`](#method.crop_length) or to a specified length
  ///
  /// # Examples
  ///
  /// ```
  /// q.crop(&[("overview", None), ("description", Some(10))]);
  /// ```
  pub fn crop(mut self, attributes: &'m [(&'m str, Option<i64>)]) -> Query<'m> {
    let crops = attributes
      .iter()
      .map(|(attribute, length)| match length {
        Some(length) => format!("{}:{}", attribute, length),
        None => attribute.to_string(),
      })
      .collect();

    self.crop = Some(crops);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#cropLength)
  ///
  /// # Arguments
  ///
  /// * `attributes` - at what length to crop attribute values
  ///
  /// # Examples
  ///
  /// ```
  /// q.crop_length(32);
  /// ```
  pub fn crop_length(mut self, length: i64) -> Query<'m> {
    self.crop_length = Some(length);
    self
  }

  /// [MeiliSearch documentation](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#attributesToRetrieve)
  ///
  /// # Arguments
  ///
  /// * `attributes` - slice of attributes to highlight
  ///
  /// # Examples
  ///
  /// ```
  /// q.highlight(&["overview"]);
  /// ```
  pub fn highlight(mut self, attributes: &'m [&'m str]) -> Query<'m> {
    self.highlight = Some(attributes);
    self
  }

  pub async fn run<R>(self) -> Result<Results<R>, Error>
  where
    R: Schema + for<'de> Deserialize<'de>,
  {
    let response = self
      .meili
      .request(Method::POST, &format!("/indexes/{}/search", self.index))
      .json(&self)
      .send()
      .await
      .map_err(|err| Error::UpstreamError(err))?;

    match response.status() {
      StatusCode::OK => {
        let response = response
          .json::<Results<R>>()
          .await
          .map_err(|err| Error::UpstreamError(err))?;

        Ok(response)
      }

      _ => {
        let error = response
          .json::<QueryError>()
          .await
          .map_err(|err| Error::UpstreamError(err))?;

        Err(Error::InvalidQuery(error))
      }
    }
  }
}
