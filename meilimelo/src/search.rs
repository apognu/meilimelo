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
/// # use meilimelo::prelude::*;
/// #
/// # #[meilimelo::schema]
/// # struct Employee;
/// #
/// # #[tokio::main]
/// # async fn main() {
/// let meili = MeiliMelo::new("host");
///
/// let results = meili.search("employees")
///   .query("johnson")
///   .facets(FacetBuilder::new("company", "ACME Corp").build())
///   .distribution(&["roles"])
///   .limit(10)
///   .run::<Employee>()
///   .await;
/// # }
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

/// Enum representing an attribute crop instruction
pub enum Crop<'a> {
  /// Crop the specified attribute at the global [`cropLength`](struct.Query.html#method.crop_length) length
  Attr(&'a str),
  /// Crop the specified attribute at the given length
  At(&'a str, i64),
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").query("streamer");
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").filters("company = ACME AND age > 23");
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").limit(10);
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").offset(20);
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index")
  ///   .facets(FacetBuilder::new("company", "ACME Corp")
  ///   .or("company", "Big Corp")
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").retrieve(&["firstname", "lastname"]);
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").distribution(&["firstname", "lastname"]);
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index")
  ///   .crop(&[
  ///      Crop::Attr("overview"),
  ///      Crop::At("description", 10)
  ///    ]);
  /// ```
  pub fn crop(mut self, attributes: &'m [Crop]) -> Query<'m> {
    let crops = attributes
      .iter()
      .map(|spec| match spec {
        Crop::Attr(attribute) => attribute.to_string(),
        Crop::At(attribute, length) => format!("{}:{}", attribute, length),
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").crop_length(32);
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
  /// # use meilimelo::prelude::*;
  /// #
  /// MeiliMelo::new("host").search("index").highlight(&["overview"]);
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

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn index() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees");

    assert_eq!(query.index, "employees");
    assert_eq!(query.query, None);
  }

  #[test]
  fn query() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").query("skywalker");

    assert_eq!(query.query, Some("skywalker"));
  }

  #[test]
  fn filters() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").filters("name = skywalker");

    assert_eq!(query.filters, Some("name = skywalker"));
  }

  #[test]
  fn limit_offset() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").limit(10).offset(20);

    assert_eq!(query.limit, Some(10));
    assert_eq!(query.offset, Some(20));
  }

  #[test]
  fn facets() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").facets(
      FacetBuilder::new("company", "ACME")
        .or("company", "Corp")
        .and("department", "IT")
        .build(),
    );

    assert_eq!(
      query.facets,
      Some(vec![
        vec!["company:ACME".to_string(), "company:Corp".to_string()],
        vec!["department:IT".to_string()]
      ])
    );
  }

  #[test]
  fn retrieve() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").retrieve(&["firstname", "lastname"]);

    assert_eq!(query.retrieve, Some(&["firstname", "lastname"] as &[&str]))
  }

  #[test]
  fn distribution() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").distribution(&["age"]);

    assert_eq!(query.distribution, Some(&["age"] as &[&str]));
  }

  #[test]
  fn crop() {
    let meili = MeiliMelo::new("");
    let query = meili
      .search("employees")
      .crop(&[Crop::Attr("firstname"), Crop::At("lastname", 10)]);

    assert_eq!(
      query.crop,
      Some(vec!["firstname".to_string(), "lastname:10".to_string()])
    );
  }

  #[test]
  fn crop_length() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").crop_length(32);

    assert_eq!(query.crop_length, Some(32));
  }

  #[test]
  fn highlight() {
    let meili = MeiliMelo::new("");
    let query = meili.search("employees").highlight(&["overview", "bio"]);

    assert_eq!(query.highlight, Some(&["overview", "bio"] as &[&str]));
  }
}
