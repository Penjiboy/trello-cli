#[macro_use]
extern crate lazy_static;

use mongodb::{Client, options::ClientOptions };

mod data;
use crate::data::data_repository::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("I guess we're doing things in rust now");
    println!("Let's first try to get all the boards");
    println!();

    // let mongo_result = test_mongo_connection().await;
    // assert_eq!(mongo_result.is_ok(), true);
    // println!();

    DataRepository::init();
    let boards_result = DataRepository::get_all_boards().await;

    match boards_result {
        Ok(boards) => {
            println!("We got boards!");
            println!("There are {} boards all together", boards.len());
        },

        Err(why) => println!("Failed to get all boards: {}", why),
    };
}

async fn test_mongo_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a connection string into an options struct
    let mut client_options = ClientOptions::parse("mongodb://root:rootpassword@localhost:32392").await?;

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