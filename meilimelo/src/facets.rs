/// Utility to help build facet filters using the builder pattern
///
/// Calling `build()` will produce a `Facets` struct that can be fed to `Query`'s [`facets()`](struct.Query.html#method.facets).
///
/// # Examples
/// ```
/// FacetBuilder::new("company", "ACME Corp")
///   .or("company", "Big Corp")
///   .and("roles", "Tech")
///   .build();
/// ```
pub struct FacetBuilder {
  current: Vec<String>,
  accumulator: Vec<Vec<String>>,
}

pub struct Facets {
  accumulator: Vec<Vec<String>>,
}

impl FacetBuilder {
  pub fn new(key: &str, value: &str) -> FacetBuilder {
    FacetBuilder {
      current: vec![format!("{}:{}", key, value)],
      accumulator: vec![],
    }
  }

  pub fn or(mut self, key: &str, value: &str) -> FacetBuilder {
    self.current.push(format!("{}:{}", key, value));
    self
  }

  pub fn and(mut self, key: &str, value: &str) -> FacetBuilder {
    self.accumulator.push(self.current);
    self.current = vec![format!("{}:{}", key, value)];
    self
  }

  pub fn build(mut self) -> Facets {
    self.accumulator.push(self.current);

    Facets {
      accumulator: self.accumulator.drain(..).collect(),
    }
  }
}

impl Facets {
  pub(crate) fn get(self) -> Vec<Vec<String>> {
    self.accumulator
  }
}
