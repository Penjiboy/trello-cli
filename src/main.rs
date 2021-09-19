use std::fs::File;
use std::io::Read;

use serde_json::{Value};
use reqwest;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    println!("I guess we're doing things in rust now");
    println!("Let's first try to get all the boards");

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
    let test_response = reqwest::blocking::get("https://httpbin.org/ip")?;
        // .json::<HashMap<String, String>>()?;
    println!("{:#?}", test_response);

    let mut url = String::from("https://api.trello.com/1");
    
    let url_path: String = format!("/members/me/boards?key={key}&token={token}", key=key, token=token);
    url.push_str(&url_path);
    let trello_response = reqwest::blocking::get(&url)?.text()?;

    let boards: Value = serde_json::from_str(&trello_response)?;
    println!("Boards: {:#?}", boards);

    // Print the specific board info
    let first_board_name = boards[0]["name"].as_str().unwrap_or_default();
    println!("First board name: {}", first_board_name);

    Ok(())
}
