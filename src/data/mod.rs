use datetime;
use serde::{Deserialize, Serialize};

pub mod data_repository;
mod trello_data_store;
mod mongo_data_store;

#[derive(Clone, Serialize, Deserialize)]
pub struct ID {
    pub trello_id: Option<String>,
    pub local_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardLabel {
    pub id: ID,
    pub board_id: String,
    pub name: String,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardComment {
    pub id: ID,
    pub text: String,
    pub commenter_name: String,
    pub comment_time_instant_ms: i16, // milliseconds of an instant
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardChecklistTask {
    pub id: ID,
    pub name: String,
    pub is_complete: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardChecklist {
    pub id: ID,
    pub name: String,
    pub tasks: Vec<CardChecklistTask>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub due_date_instant_ms: i16,
    pub labels: Vec<CardLabel>,
    pub comments: Vec<CardComment>,
    pub checklists: Vec<CardChecklist>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardDueDate {
    pub card: Card,
    pub due_date_instant_ms: i16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BoardList {
    pub id: ID,
    pub name: String,
    pub board: Board,
    pub cards: Vec<Card>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: ID,
    pub name: String,
}

#[derive(Debug)]
pub struct NotImplError {}

impl std::error::Error for NotImplError {}

impl std::fmt::Display for NotImplError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Not implemented yet!")
    }
}

#[derive(Debug)]
pub struct InvalidInputError {}

impl std::error::Error for InvalidInputError {}

impl std::fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid Input!")
    }
}