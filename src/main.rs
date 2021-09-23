use std::fs::File;
use std::io::Read;

use serde_json::{Value};
use reqwest;
use mongodb::{Client, options::ClientOptions };

mod data;
// use crate::data;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("I guess we're doing things in rust now");
    println!("Let's first try to get all the boards");
    println!();

    let trello_result = test_trello_request().await;
    assert_eq!(trello_result.is_ok(), true);
    println!();

    let mongo_result = test_mongo_connection().await;
    assert_eq!(mongo_result.is_ok(), true);
    println!();

    let card_label = data::CardLabel {
        id: data::ID {
            trello_id: String::from("test"),
            local_id: String::from("test"),
        },
        name: String::from("Test"),
        color: String::from("Red"),
    };
    println!("Card Label\nName:{name}\nColor:{color}", name=card_label.name, color=card_label.color);
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

async fn test_trello_request() -> Result<(), Box<dyn std::error::Error>> {
    // Read the API key and token
    let path_to_key = ".config/developer_api_key.txt";
    let path_to_token = ".config/developer_api_token.txt";

    let mut key_file = File::open(path_to_key).unwrap();
    let mut key: String = String::new();
    key_file.read_to_string(&mut key).unwrap();

    let mut token_file = File::open(path_to_token).unwrap();
    let mut token: String = String::new();
    token_file.read_to_string(&mut token).unwrap();

    println!("Key: {key}\nToken: {token}", key=key, token=token);

    // Send a request to Trello
    let test_response = reqwest::get("https://httpbin.org/ip").await?;
        // .json::<HashMap<String, String>>()?;
    println!("{:#?}", test_response);

    let mut url = String::from("https://api.trello.com/1");
    
    let url_path: String = format!("/members/me/boards?key={key}&token={token}", key=key, token=token);
    url.push_str(&url_path);
    let trello_response = reqwest::get(&url).await?.text().await?;

    let boards: Value = serde_json::from_str(&trello_response)?;
    println!("Boards: {:#?}", boards);

    // Print the specific board info
    let first_board_name = boards[0]["name"].as_str().unwrap_or_default();
    println!("First board name: {}", first_board_name);

    Ok(())
}