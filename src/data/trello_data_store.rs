use std::fs::File;
use std::io::Read;
use std::sync::Once;
use std::str::FromStr;
use std::collections::HashMap;

use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use reqwest;
use serde_json::{Map, Value, json};
use chrono::{DateTime, TimeZone, Utc, Local};

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

    fn parse_board_from_json(board_json: &Value) -> Result<Board, Box<dyn std::error::Error>> {
        let board_object = board_json.as_object().unwrap();
        let trello_id = board_object.get("id").unwrap().as_str().unwrap();
        let board_name = board_object.get("name").unwrap().as_str().unwrap();
        let board = Board {
            id: ID {
                trello_id: Some(String::from(trello_id)),
                local_id: None,
            },
            name: String::from(board_name),
        };

        Ok(board)
    }

    fn parse_label_from_json(label_json: &Value) -> Result<CardLabel, Box<dyn std::error::Error>> {
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

    fn parse_list_from_json(list_json: &Value) -> Result<BoardList, Box<dyn std::error::Error>> {
        let list_object = list_json.as_object().unwrap();
        let trello_id = Some(String::from(
            list_object.get("id").unwrap().as_str().unwrap(),
        ));
        let board_id = String::from(list_object.get("idBoard").unwrap().as_str().unwrap());
        let list_name = String::from(list_object.get("name").unwrap().as_str().unwrap());

        let list = BoardList {
            id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            board_id: ID {
                trello_id: Some(board_id),
                local_id: None,
            },
            name: list_name,
        };

        Ok(list)
    }

    fn parse_card_from_json(card_json: &Value) -> Result<Card, Box<dyn std::error::Error>> {
        let card_object = card_json.as_object().unwrap();
        let trello_id = Some(String::from(
            card_object.get("id").unwrap().as_str().unwrap(),
        ));
        let card_name = String::from(card_object.get("name").unwrap().as_str().unwrap());
        let id_list = String::from(card_object.get("idList").unwrap().as_str().unwrap());
        
        let mut id_checklists: Vec<ID> = Vec::new();
        for checklist_id in card_object.get("idChecklists").unwrap().as_array().unwrap() {
            let id = ID {
                trello_id: Some(String::from(checklist_id.as_str().unwrap())),
                local_id: None
            };
            id_checklists.push(id);
        }

        let mut id_labels: Vec<ID> = Vec::new();
        for label_id in card_object.get("idLabels").unwrap().as_array().unwrap() {
            let id = ID {
                trello_id: Some(String::from(label_id.as_str().unwrap())),
                local_id: None
            };
            id_labels.push(id);
        }

        let description: String = card_object.get("desc").unwrap().as_str().unwrap().to_string();

        let due_complete: bool = card_object.get("dueComplete").unwrap().as_bool().unwrap();

        let due_string: Option<&str> = card_object.get("due").unwrap().as_str();
        let due_instant_seconds: i64 = if due_string.is_none() {
            0
        } else {
            let due_datetime: DateTime<Utc> = due_string.unwrap().parse::<DateTime<Utc>>().unwrap();
            due_datetime.timestamp()
        };

        let card = Card {
            id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            name: card_name,
            description: description,
            due_complete: due_complete,
            due_date_instant_seconds: due_instant_seconds,
            list_id: ID {
                trello_id: Some(id_list),
                local_id: None,
            },
            checklists_ids: id_checklists,
            label_ids: id_labels,
        };

        Ok(card)

    }

    fn parse_comment_from_json(comment_json: &Value) -> Result<CardComment, Box<dyn std::error::Error>> {
        let comment_object = comment_json.as_object().unwrap();
        let trello_id = Some(String::from(
            comment_object.get("id").unwrap().as_str().unwrap(),
        ));
        let date_string: Option<&str> = comment_object.get("date").unwrap().as_str();
        let date_instant_seconds: i64 = if date_string.is_none() {
            0
        } else {
            let date_datetime: DateTime<Utc> = date_string.unwrap().parse::<DateTime<Utc>>().unwrap();
            date_datetime.timestamp()
        };

        let commenter_name: String = comment_object.get("memberCreator").unwrap().as_object().unwrap().get("fullName").unwrap().as_str().unwrap().to_string();
        let comment_text: String = comment_object.get("data").unwrap().as_object().unwrap().get("text").unwrap().as_str().unwrap().to_string();

        let comment = CardComment {
            id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            commenter_name: commenter_name,
            text: comment_text,
            comment_time_instant_seconds: date_instant_seconds
        };

        Ok(comment)
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
            let board: Board = TrelloDataStore::parse_board_from_json(&board_json)?;
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
        TrelloDataStore::parse_board_from_json(&board_serde)
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
        for label_json in labels.as_array().unwrap() {
            let label: CardLabel = TrelloDataStore::parse_label_from_json(&label_json)?;
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
        TrelloDataStore::parse_label_from_json(&label_json)
    }

    async fn create_board_label(
        board_id: &str,
        name: &str,
        color: &str
    ) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/boards/{board_id}/labels?key={key}&token={token}&name={name}&color={color}",
                board_id = board_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
                name = name,
                color = color
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.post(&full_url).send().await?;
        let response_text = response.text().await?;
        let label_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_label_from_json(&label_json)
    }

    async fn get_all_board_lists(board_id: &str) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/boards/{id}/lists?key={key}&token={token}",
                id = board_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let lists: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<BoardList> = Vec::new();
        for list_json in lists.as_array().unwrap() {
            let list: BoardList = TrelloDataStore::parse_list_from_json(&list_json)?;
            result.push(list);
        }

        Ok(result)
    }

    async fn create_board_list(board_id: &str, name: &str) -> Result<BoardList, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/boards/{board_id}/lists?key={key}&token={token}&name={name}",
                board_id = board_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
                name = name,
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.post(&full_url).send().await?;
        let response_text = response.text().await?;
        let list_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_list_from_json(&list_json)
    }

    async fn get_all_list_cards(list_id: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/lists/{id}/cards?key={key}&token={token}",
                id = list_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let cards: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<Card> = Vec::new();
        for card_json in cards.as_array().unwrap() {
            let card: Card = TrelloDataStore::parse_card_from_json(&card_json)?;
            result.push(card);
        }

        Ok(result)
    }

    async fn create_list_card(list_id: &str, name: &str) -> Result<Card, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/?idList={id}&key={key}&token={token}&name={name}",
                id = list_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
                name = name,
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.post(&full_url).send().await?;
        let response_text = response.text().await?;
        let card_json: Value = serde_json::from_str(&response_text)?;

        TrelloDataStore::parse_card_from_json(&card_json)
    }

    async fn update_card(card: &Card) -> Result<Card, Box<dyn std::error::Error>> {
        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/cards/{id}?key={key}&token={token}",
                id = card.id.trello_id.clone().unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let label_trello_ids = card.label_ids.iter().map(|id| id.trello_id.clone().unwrap_or("".to_string())).collect::<Vec<_>>();
        let due_date: Option<DateTime<Utc>> = if card.due_date_instant_seconds == 0 {
            None
        } else {
            Some(Utc.timestamp(card.due_date_instant_seconds, 0))
        };

        let due_string: Value = if due_date.is_some() {
            let due_datetime = due_date.unwrap();
            Value::String(format!("{:?}", due_datetime))
        } else {
            Value::Null
        };

        let request_body = json!({
            "desc": card.description.clone(),
            "idList": card.list_id.trello_id.clone().unwrap(),
            "name": card.name.clone(),
            "dueComplete": card.due_complete,
            "idLabels": label_trello_ids,
            "due": due_string,
            "dueComplete": card.due_complete
        });

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.put(&full_url).json(&request_body).send().await?;
        let response_text = response.text().await?;
        let card_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_card_from_json(&card_json)
    }

    async fn get_card_comments(card_id: &str) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {

        let mut url_path: String = String::from("");
        unsafe {
            url_path = format!(
                "/cards/{id}/actions?key={key}&token={token}&filter=commentCard",
                id = card_id,
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let comments: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<CardComment> = Vec::new();
        for comment_json in comments.as_array().unwrap() {
            let comment: CardComment = TrelloDataStore::parse_comment_from_json(&comment_json)?;
            result.push(comment);
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
