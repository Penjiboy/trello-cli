use std::fs::File;
use std::io::Read;
use std::sync::Once;

use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use reqwest;
use serde_json::{Value, json};
use chrono::{DateTime, TimeZone, Utc};

static START: Once = Once::new();

const URL_BASE: &str = "https://api.trello.com/1";
const PATH_TO_KEY: &str = ".config/developer_api_key.txt";
const PATH_TO_TOKEN: &str = ".config/developer_api_token.txt";

static mut key: Option<String> = None;
static mut token: Option<String> = None;

#[derive(Clone)]
pub struct TrelloDataStore {}

impl TrelloDataStore {
    pub fn init(config: Option<Value>) {
        let mut _key = String::from("");
        let mut _token = String::from("");

        if config.is_some() {
            let config_object = config.as_ref().unwrap().as_object().unwrap();
            _key = config_object.get("trello").unwrap().as_object().unwrap().get("developer_api_key").unwrap().as_str().unwrap().to_string();
            _token = config_object.get("trello").unwrap().as_object().unwrap().get("developer_api_token").unwrap().as_str().unwrap().to_string();
        } else {
            let mut key_file = File::open(PATH_TO_KEY.to_string()).unwrap();
            key_file.read_to_string(&mut _key).unwrap();

            let mut token_file = File::open(PATH_TO_TOKEN.to_string()).unwrap();
            token_file.read_to_string(&mut _token).unwrap();
        }

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
            _id: ID {
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
            _id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            board_id: ID {
                trello_id: Some(String::from(board_id)),
                local_id: None
            },
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
            _id: ID {
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
            _id: ID {
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
            _id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            commenter_name: commenter_name,
            text: comment_text,
            comment_time_instant_seconds: date_instant_seconds
        };

        Ok(comment)
    }

    fn parse_checklist_from_json(checklist_json: &Value) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        let checklist_object = checklist_json.as_object().unwrap();
        let trello_id = Some(String::from(
            checklist_object.get("id").unwrap().as_str().unwrap(),
        ));

        let name: String = checklist_object.get("name").unwrap().as_str().unwrap().to_string();
        let card_trello_id = Some(String::from(
            checklist_object.get("idCard").unwrap().as_str().unwrap(),
        ));

        let checklist = CardChecklist {
            _id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            name: name,
            card_id: ID {
                trello_id: card_trello_id,
                local_id: None
            },
        };

        Ok(checklist)
    }

    fn parse_checklist_task_from_json(checklist_task_json: &Value) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let checklist_task_object = checklist_task_json.as_object().unwrap();
        let trello_id = Some(String::from(
            checklist_task_object.get("id").unwrap().as_str().unwrap(),
        ));

        let name: String = checklist_task_object.get("name").unwrap().as_str().unwrap().to_string();
        let checklist_trello_id = Some(String::from(
            checklist_task_object.get("idChecklist").unwrap().as_str().unwrap(),
        ));
        let state: String = checklist_task_object.get("state").unwrap().as_str().unwrap().to_string();
        let is_complete: bool = state.eq_ignore_ascii_case("complete");

        let checklist_task = CardChecklistTask {
            _id: ID {
                trello_id: trello_id,
                local_id: None,
            },
            name: name,
            checklist_id: ID {
                trello_id: checklist_trello_id,
                local_id: None
            },
            is_complete: is_complete,
        };

        Ok(checklist_task)
    }
}

#[async_trait]
impl DataStore for TrelloDataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let url_path: String;
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
        let url_path: String;
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
        let url_path: String;
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

    async fn delete_board_label(label_id: ID) -> Result<(), Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/labels/{id}?key={key}&token={token}",
                id = label_id.trello_id.unwrap(),
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
        label_id: ID,
        name: &str,
        color: &str,
    ) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/labels/{id}?key={key}&token={token}&name={name}&color={color}",
                id = label_id.trello_id.unwrap(),
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
        board_id: ID,
        name: &str,
        color: &str
    ) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/boards/{board_id}/labels?key={key}&token={token}&name={name}&color={color}",
                board_id = board_id.trello_id.unwrap(),
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

    async fn get_all_board_lists(board_id: ID) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/boards/{id}/lists?key={key}&token={token}",
                id = board_id.trello_id.unwrap(),
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

    async fn create_board_list(board_id: ID, name: &str) -> Result<BoardList, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/boards/{board_id}/lists?key={key}&token={token}&name={name}",
                board_id = board_id.trello_id.unwrap(),
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

    async fn get_all_list_cards(list_id: ID) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/lists/{id}/cards?key={key}&token={token}",
                id = list_id.trello_id.unwrap(),
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

    async fn create_list_card(list_id: ID, name: &str) -> Result<Card, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/?idList={id}&key={key}&token={token}&name={name}",
                id = list_id.trello_id.unwrap(),
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
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/{id}?key={key}&token={token}",
                id = card._id.trello_id.clone().unwrap(),
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

    async fn get_card_comments(card_id: ID) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {

        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/{id}/actions?key={key}&token={token}&filter=commentCard",
                id = card_id.trello_id.unwrap(),
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

    async fn add_card_comment(card_id: ID, text: &str) -> Result<CardComment, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/{id}/actions/comments?key={key}&token={token}",
                id = card_id.trello_id.unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let request_body = json!({
            "text": text
        });

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.post(&full_url).json(&request_body).send().await?;
        let response_text = response.text().await?;
        let comment_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_comment_from_json(&comment_json)
    }

    async fn get_card_checklists(card_id: ID) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/{id}/checklists?key={key}&token={token}",
                id = card_id.trello_id.unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let checklists: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<CardChecklist> = Vec::new();
        for checklist_json in checklists.as_array().unwrap() {
            let checklist: CardChecklist = TrelloDataStore::parse_checklist_from_json(&checklist_json)?;
            result.push(checklist);
        }

        Ok(result)

    }

    async fn create_card_checklist(card_id: ID, name: &str) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/{id}/checklists?key={key}&token={token}",
                id = card_id.trello_id.unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let request_body = json!({
            "name": name
        });

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.post(&full_url).json(&request_body).send().await?;
        let response_text = response.text().await?;
        let checklist_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_checklist_from_json(&checklist_json)
    }

    async fn get_checklist_tasks(checklist_id: ID) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/checklists/{id}/checkItems?key={key}&token={token}",
                id = checklist_id.trello_id.unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let trello_response = reqwest::get(&full_url).await?.text().await?;

        let tasks: Value = serde_json::from_str(&trello_response)?;

        let mut result: Vec<CardChecklistTask> = Vec::new();
        for task_json in tasks.as_array().unwrap() {
            let task: CardChecklistTask = TrelloDataStore::parse_checklist_task_from_json(&task_json)?;
            result.push(task);
        }

        Ok(result)
    }

    async fn create_checklist_task(checklist_id: ID, name: &str) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/checklists/{id}/checkItems?key={key}&token={token}",
                id = checklist_id.trello_id.unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let request_body = json!({
            "name": name
        });

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.post(&full_url).json(&request_body).send().await?;
        let response_text = response.text().await?;
        let task_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_checklist_task_from_json(&task_json)
    }

    async fn update_checklist_task(card_id: ID, task: &CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let url_path: String;
        unsafe {
            url_path = format!(
                "/cards/{card_id}/checkitem/{task_id}?key={key}&token={token}",
                card_id = card_id.trello_id.unwrap(),
                task_id = task._id.trello_id.clone().unwrap(),
                key = key.clone().unwrap(),
                token = token.clone().unwrap(),
            );
        }

        let state = if task.is_complete {
            "complete"
        } else {
            "incomplete"
        };
        let request_body = json!({
            "name": task.name.clone(),
            "state": state
        });

        let mut full_url = String::from(URL_BASE);
        full_url.push_str(&url_path);
        let client = reqwest::Client::new();
        let response = client.put(&full_url).json(&request_body).send().await?;
        let response_text = response.text().await?;
        let task_json: Value = serde_json::from_str(&response_text)?;
        TrelloDataStore::parse_checklist_task_from_json(&task_json)
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
