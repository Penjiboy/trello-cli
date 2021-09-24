use datetime;

pub mod data_repository;
mod trello_data_store;

pub struct ID {
    pub trello_id: Option<String>,
    pub local_id: Option<String>,
}

pub struct CardLabel {
    pub name: String,
    pub color: String,
}

pub struct CardComment {
    pub id: ID,
    pub text: String,
    pub commenter_name: String,
    pub comment_time: datetime::LocalDateTime,
}

pub struct CardChecklistTask {
    pub id: ID,
    pub name: String,
    pub is_complete: bool,
}

pub struct CardChecklist {
    pub id: ID,
    pub name: String,
    pub tasks: Vec<CardChecklistTask>,
}

pub struct Card {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub due_date: datetime::LocalDateTime,
    pub labels: Vec<CardLabel>,
    pub comments: Vec<CardComment>,
    pub checklists: Vec<CardChecklist>,
}

pub struct CardDueDate {
    pub card: Card,
    pub due_date: datetime::LocalDateTime,
}

pub struct BoardList {
    pub id: ID,
    pub name: String,
    pub board: Board,
    pub cards: Vec<Card>,
}

pub struct Board {
    pub id: ID,
    pub name: String,
    pub labels: Vec<CardLabel>,
}