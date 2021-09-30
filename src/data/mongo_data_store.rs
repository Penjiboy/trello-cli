use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client, Database, Collection};
use serde_json::{Map, Value};

pub struct MongoDataStore {
    db: Database
}

impl MongoDataStore {
    async fn new() -> MongoDataStore {
        // Parse a connection string into an options struct
        let mut client_options =
            ClientOptions::parse("mongodb://root:rootpassword@localhost:32392").await.unwrap();

        // Manually set an option
        client_options.app_name = Some(String::from("TrelloData"));

        // Get a handle to the deployment
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("trelloData");
        
        let mds = MongoDataStore {
            db: db
        };
        mds
    }
}

#[async_trait]
impl DataStore for MongoDataStore {
    async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let boards_collection: Collection<Board> = self.db.collection::<Board>("boards");
    }
}

async fn test_mongo_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://root:rootpassword@localhost:32392").await?;

    // Manually set an option
    client_options.app_name = Some(String::from("TrelloData"));

    // Get a handle to the deployment
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment
    println!("Database names: ");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database
    let db = client.database("trelloData");

    // List the names of the collections in that database
    println!("Collections in trelloData: ");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}
