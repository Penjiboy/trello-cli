use std::fs::File;
use std::io::Read;

use crate::data::data_repository::DataStore;
use crate::data::*;

use reqwest;
use serde_json::{Value, Map};
use async_trait::async_trait;

const URL_BASE: &str = "https://api.trello.com/1";
const PATH_TO_KEY: &str = ".config/developer_api_key.txt";
const PATH_TO_TOKEN: &str = ".config/developer_api_token.txt";

lazy_static! {
    #[derive(Debug)]
    static ref KEY: String = {
        let mut key_file = File::open(PATH_TO_KEY).unwrap();
        let mut key = String::from("");
        key_file.read_to_string(&mut key).unwrap();
        key
    };

    #[derive(Debug)]
    static ref TOKEN: String = {
        let mut token_file = File::open(PATH_TO_TOKEN).unwrap();
        let mut token = String::from("");
        token_file.read_to_string(&mut token).unwrap();
        token
    };
}

pub struct TrelloDataStore;

#[async_trait]
impl DataStore for TrelloDataStore {
    fn init() {
        println!("Key: {:?}\nToken: {:?}", *KEY, *TOKEN);
    }

    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let url_path: String = format!(
            "/members/me/boards?key={key}&token={token}",
            key = *KEY,
            token = *TOKEN
        );
        
        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let boards: Value = serde_json::from_str(&trello_response)?;

        // Print the specific board info
        let first_board_name = boards[0]["name"].as_str().unwrap_or_default();
        println!("First board name: {}", first_board_name);

        let mut result: Vec<Board> = Vec::new();
        for board_json in boards.as_array().unwrap() {
            // board_json["labelNames"].as_array().unwrap().map()

            let label_names_map: &Map<String, Value> = board_json["labelNames"].as_object().unwrap();
            let mut card_labels: Vec<CardLabel> = Vec::new();
            for color_key in label_names_map.keys() {
                let label_name: String = String::from(label_names_map.get(color_key).unwrap().as_str().unwrap());
                let label_color: String = String::from(color_key);
                let card_label = CardLabel {
                    color: label_color,
                    name: label_name
                };

                card_labels.push(card_label);
            }

            let board_object = board_json.as_object().unwrap();
            let trello_id = Some(String::from(board_object.get("id").unwrap().as_str().unwrap()));
            let board_name = String::from(board_object.get("name").unwrap().as_str().unwrap());
            let board = Board {
                id: ID {
                    trello_id: trello_id,
                    local_id: None,
                },
                name: board_name,
                labels: card_labels
            };

            result.push(board);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        TrelloDataStore::init();
        println!("Key: {:?}\nToken: {:?}", *KEY, *TOKEN);
        assert_ne!(*KEY, String::from(""));
        assert_ne!(*TOKEN, String::from(""));
    }
}
