use std::{collections::HashMap, iter::IntoIterator};

#[derive(Debug, Deserialize)]
pub struct Results<T> {
    pub query: String,
    #[serde(rename = "exhaustiveNbHits")]
    pub exhaustive_hits: bool,
    #[serde(rename = "nbHits")]
    pub hits: i64,
    #[serde(rename = "exhaustiveFacetsCount")]
    pub exhaustive_facets: Option<bool>,
    #[serde(rename = "facetsDistribution")]
    pub distribution: Option<HashMap<String, HashMap<String, i64>>>,
    pub limit: i64,
    pub offset: i64,
    #[serde(rename = "processingTimeMs")]
    pub duration: i64,

    #[serde(rename = "hits")]
    pub results: Vec<T>,
}

impl<T> IntoIterator for Results<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

impl<'i, T> IntoIterator for &'i Results<T> {
    type Item = &'i T;
    type IntoIter = std::slice::Iter<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}
