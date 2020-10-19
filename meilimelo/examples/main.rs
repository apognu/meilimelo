use meilimelo::prelude::*;

#[meilimelo::schema]
struct Employee {
  firstname: String,
  lastname: String,
  roles: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let meili = MeiliMelo::new("http://meilisearch.example.com:7700").with_secret_key("abcdef");
  let query = meili.search("persons").query("johnson").run::<Employee>();

  for index in &meili.indices().await? {
    println!("{}", index.name);
  }

  let people = query.await?;

  println!("Hits: {}", people.hits);

  for person in &people {
    println!("{} {}", person.firstname, person.lastname);
  }

  Ok(())
}
