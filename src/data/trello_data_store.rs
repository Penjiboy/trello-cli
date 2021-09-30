use std::fs::File;
use std::io::Read;
use std::sync::Once;

use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use reqwest;
use serde_json::{Map, Value};

static START: Once = Once::new();

const URL_BASE: &str = "https://api.trello.com/1";
const PATH_TO_KEY: &str = ".config/developer_api_key.txt";
const PATH_TO_TOKEN: &str = ".config/developer_api_token.txt";

static mut key: Option<String> = None;
static mut token: Option<String> = None;

#[derive(Clone)]
pub struct TrelloDataStore {}

impl TrelloDataStore {
    pub fn init(key_path: Option<String>, token_path: Option<String>) {
        let mut key_file = File::open(key_path.unwrap_or(PATH_TO_KEY.to_string())).unwrap();
        let mut _key = String::from("");
        key_file.read_to_string(&mut _key).unwrap();

        let mut token_file = File::open(token_path.unwrap_or(PATH_TO_TOKEN.to_string())).unwrap();
        let mut _token = String::from("");
        token_file.read_to_string(&mut _token).unwrap();

        unsafe {
            START.call_once(|| {
                key = Some(_key);
                token = Some(_token);
            });
        }
    }
}

#[async_trait]
impl DataStore for TrelloDataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/members/me/boards?key={key}&token={token}",
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let boards: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<Board> = Vec::new();
        for board_json in boards.as_array().unwrap() {
            let label_names_map: &Map<String, Value> =
                board_json["labelNames"].as_object().unwrap();
            let mut card_labels: Vec<CardLabel> = Vec::new();
            for color_key in label_names_map.keys() {
                let label_name: String =
                    String::from(label_names_map.get(color_key).unwrap().as_str().unwrap());
                let label_color: String = String::from(color_key);
                let card_label = CardLabel {
                    color: label_color,
                    name: label_name,
                };

                card_labels.push(card_label);
            }

            let board_object = board_json.as_object().unwrap();
            let trello_id = Some(String::from(
                board_object.get("id").unwrap().as_str().unwrap(),
            ));
            let board_name = String::from(board_object.get("name").unwrap().as_str().unwrap());
            let board = Board {
                id: ID {
                    trello_id: trello_id,
                    local_id: None,
                },
                name: board_name,
                labels: card_labels,
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
        unsafe {
            println!("Key: {:?}\nToken: {:?}", key.clone().unwrap(), token.clone().unwrap());
            assert_ne!(key.clone().unwrap(), String::from(""));
            assert_ne!(token.clone().unwrap(), String::from(""));
        }
    }
}
