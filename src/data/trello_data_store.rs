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
            };

            result.push(board);
        }

        Ok(result)
    }

    async fn create_board(name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/boards?key={key}&token={token}&name={board_name}&defaultLabels=false",
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
                board_name = name
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let trello_response = client.post(&full_url).send().await?.text().await?;

        let board_serde: Value = serde_json::from_str(&trello_response)?;
        let board_map = board_serde.as_object().unwrap();
        let trello_id = board_map.get("id").unwrap().as_str().unwrap();
        let board_name = board_map.get("name").unwrap().as_str().unwrap();
        let board = Board {
            id: ID {
                trello_id: Some(String::from(trello_id)),
                local_id: None,
            },
            name: String::from(board_name),
        };

        Ok(board)
    }

    async fn get_all_board_labels(
        board_id: &str,
    ) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/boards/{id}/labels?key={key}&token={token}",
                id = board_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let labels: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<CardLabel> = Vec::new();
        for labels_json in labels.as_array().unwrap() {
            let label_object = labels_json.as_object().unwrap();
            let trello_id = Some(String::from(
                label_object.get("id").unwrap().as_str().unwrap(),
            ));
            let label_name = String::from(label_object.get("name").unwrap().as_str().unwrap());
            let label_color = String::from(label_object.get("color").unwrap().as_str().unwrap());

            let label = CardLabel {
                id: ID {
                    trello_id: trello_id,
                    local_id: None,
                },
                board_id: String::from(board_id),
                name: label_name,
                color: label_color,
            };
            result.push(label);
        }

        Ok(result)
    }

    async fn delete_board_label(label_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/labels/{id}?key={key}&token={token}",
                id = label_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.delete(&full_url).send().await?;
        let response_status = response.status();
        if response_status.is_success() {
            Ok(())
        } else {
            Err(Box::new(InvalidInputError {
                message: Some(format!(
                    "Failed to delete label. {}",
                    response_status.canonical_reason().unwrap_or("")
                )),
            }))
        }
    }

    async fn update_board_label(
        label_id: &str,
        name: &str,
        color: &str,
    ) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/labels/{id}?key={key}&token={token}&name={name}&color={color}",
                id = label_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
                name = name,
                color = color
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.put(&full_url).send().await?;
        let response_text = response.text().await?;
        let label_json: Value = serde_json::from_str(&response_text)?;
        let label_object = label_json.as_object().unwrap();
        let trello_id = Some(String::from(
            label_object.get("id").unwrap().as_str().unwrap(),
        ));
        let board_id = String::from(label_object.get("idBoard").unwrap().as_str().unwrap());
        let label_name = String::from(label_object.get("name").unwrap().as_str().unwrap());
        let label_color = String::from(label_object.get("color").unwrap().as_str().unwrap());

        let label = CardLabel {
            id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            board_id: String::from(board_id),
            name: label_name,
            color: label_color,
        };

        Ok(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        unsafe {
            println!(
                "Key: {:?}\nToken: {:?}",
                key.clone().unwrap(),
                token.clone().unwrap()
            );
            assert_ne!(key.clone().unwrap(), String::from(""));
            assert_ne!(token.clone().unwrap(), String::from(""));
        }
    }
}
