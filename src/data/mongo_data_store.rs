use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use serde_json::{Map, Value};

use std::sync::Once;

static START: Once = Once::new();

static mut db: Option<Database> = None;

pub struct MongoDataStore {}

impl MongoDataStore {
    pub async fn init() {
        // Parse a connection string into an options struct
        let mut client_options =
            ClientOptions::parse("mongodb://root:rootpassword@localhost:32392")
                .await
                .unwrap();

        // Manually set an option
        client_options.app_name = Some(String::from("TrelloData"));

        // Get a handle to the deployment
        let client = Client::with_options(client_options).unwrap();
        let _db = client.database("trelloData");

        unsafe {
            START.call_once(|| {
                db = Some(_db);
            });
        }
    }
}

#[async_trait]
impl DataStore for MongoDataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let boards: Vec<Board>;

        unsafe {
            let boards_collection: Collection<Board> = db.clone().unwrap().collection::<Board>("boards");
            let cursor = boards_collection.find(None, None).await?;
            boards = cursor.try_collect().await?;
        }

        for board in &boards {
            println!("{}", board.name);
        }
        Ok(boards)
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
    let _db = client.database("trelloData");

    // List the names of the collections in that database
    println!("Collections in trelloData: ");
    for collection_name in _db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}
