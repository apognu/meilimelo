# meilimelo

![MIT License](https://img.shields.io/badge/license-MIT-informational?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/meilimelo?style=flat-square)

Meilimelo is a simple library to perform queries against [Meilisearch](https://github.com/meilisearch/MeiliSearch).

## Example

```rust
use meilimelo::prelude::*;

#[meilimelo::schema]
struct Employee {
  firstname: String,
  lastname: String,
  roles: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let meili = MeiliMelo::new("https://meilisearch.example.com:7700")
      .with_secret_key("helloworld");

    let employees = meili
        .search("employees")
        .query("johnson")
        .run::<Employee>()
        .await?;

    println!("Hits: {}", people.hits);

    for person in &people {
        println!("{} {}", person.firstname, person.lastname);
    }

    Ok(())
}
```

The `meilimelo::schema` attribute macro allows for deriving your schema to something that can be used as a MeiliSearch search result (for example, automatically adding the `_formatted` sub-object when needed).

## Querying

Most of MeiliSearch's query parameters are handled by `meilimelo`. They can all be added through the request builder:

### Query, filter, limit and offset

```rust
meili
  .search("employees")
  .query("johnson")
  .filters("age > 23 AND location = Paris")
  .limit(10)
  .offset(5);
```

### Facets

```rust
meili
  .search("employees")
  .facets(FacetBuilder::new("company", "ACME Corp").or("company", "Big Corp").and("roles", "CXM").build())
  .distribution(&["roles"]);
```

### Output settings

```rust
meili
  .attributes(&["firstname", "lastname", "bio"])
  .crop(&[Crop::At("bio", 32)])
  .crop_length(10)
  .highlight(&["bio"])
  .matches(true);
```

## Index management

Basic operation on indexes can be performed:

```rust
// Creating index
meili.create_index("employees", "Employees")?;

// Listing indices
for index in &meili.indices().await? {
  println!("{}", index.name);
}
```

## Document management

You can index a collection of `Serialize` documents like so (listing and deleting documents is also supported):

```rust
// Indexing documents
let doc = Employee {
  firstname: "Luke".to_string(),
  lastname: "Skywalker".to_string()
};

meili.insert("employees", vec![doc]);

// Looping over in-order documents
for doc in &meili.list_documents::<Employee>("employees").await? {
  println!("{} {}", doc.firstname, doc.lastname);
}

// Getting document via primary key
let doc = meili.get_document::<Employee>("employees", "lskywalker").await?;

println!("{} {}", doc.firstname, doc.lastname);

// Deleting document
meili.delete_document("employees", "lskywalker").await?;
```
