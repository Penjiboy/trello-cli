use datetime;

pub struct ID {
    pub trello_id: String,
    pub local_id: String,
}

pub struct CardLabel {
    pub id: ID,
    pub name: String,
    pub color: String,
}

pub struct CardComment {
    pub id: ID,
    pub text: String,
    pub commenter_name: String,
    pub comment_time: datetime::LocalDateTime,
}
