use datetime;

pub mod data_repository;
mod trello_data_store;

#[derive(Clone)]
pub struct ID {
    pub trello_id: Option<String>,
    pub local_id: Option<String>,
}

#[derive(Clone)]
pub struct CardLabel {
    pub name: String,
    pub color: String,
}

#[derive(Clone)]
pub struct CardComment {
    pub id: ID,
    pub text: String,
    pub commenter_name: String,
    pub comment_time: datetime::LocalDateTime,
}

#[derive(Clone)]
pub struct CardChecklistTask {
    pub id: ID,
    pub name: String,
    pub is_complete: bool,
}

#[derive(Clone)]
pub struct CardChecklist {
    pub id: ID,
    pub name: String,
    pub tasks: Vec<CardChecklistTask>,
}

#[derive(Clone)]
pub struct Card {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub due_date: datetime::LocalDateTime,
    pub labels: Vec<CardLabel>,
    pub comments: Vec<CardComment>,
    pub checklists: Vec<CardChecklist>,
}

#[derive(Clone)]
pub struct CardDueDate {
    pub card: Card,
    pub due_date: datetime::LocalDateTime,
}

#[derive(Clone)]
pub struct BoardList {
    pub id: ID,
    pub name: String,
    pub board: Board,
    pub cards: Vec<Card>,
}

#[derive(Clone)]
pub struct Board {
    pub id: ID,
    pub name: String,
    pub labels: Vec<CardLabel>,
}