use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc, options::ClientOptions, options::InsertManyOptions, options::UpdateOptions, Client,
    Collection, Database,
};
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

        START.call_once(|| {
            // Get a handle to the deployment
            let client = Client::with_options(client_options).unwrap();
            let _db = client.database("trelloData");
            unsafe {
                db = Some(_db);
            }
        });
    }

    pub async fn insert_boards(boards: &Vec<Board>) {
        let boards_collection: Collection<Board>;

        unsafe {
            boards_collection = db.clone().unwrap().collection::<Board>("boards");
        }

        let insert_options = InsertManyOptions::builder().ordered(Some(false)).build();
        let insert_result = boards_collection.insert_many(boards, insert_options).await;
    }
}

#[async_trait]
impl DataStore for MongoDataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let boards: Vec<Board>;

        unsafe {
            let boards_collection: Collection<Board> =
                db.clone().unwrap().collection::<Board>("boards");
            let cursor = boards_collection.find(None, None).await?;
            boards = cursor.try_collect().await?;
        }

        for board in &boards {
            println!("{}", board.name);
        }
        Ok(boards)
    }

    async fn create_board(name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_all_board_labels(board_id: &str) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn delete_board_label(label_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn update_board_label(label_id: &str, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_board_label(board_id: &str, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
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
