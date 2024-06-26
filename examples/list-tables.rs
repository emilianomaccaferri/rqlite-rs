use rqlite_rs::prelude::*;

#[derive(FromRow)]
pub struct Table {
    name: String,
}
// Also works with unnamed fields
// #[derive(FromRow)]
// pub struct Table(String);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RqliteClientBuilder::new()
        .known_host("localhost:4001")
        .build()?;

    let query = rqlite_rs::query!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
    )?;

    let rows = client.fetch(query).await?;

    let tables = rows.into_typed::<Table>()?;
    // You can also use tuples
    // let tables = rows.into_typed::<(String,)>()?;

    for table in tables {
        println!("Table: {}", table.name);
        //println!("Table: {}", table.0);
    }

    Ok(())
}
