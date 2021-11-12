use crate::data::mongo_data_store::MongoDataStore;
use crate::data::trello_data_store::TrelloDataStore;
use crate::data::*;

use async_trait::async_trait;

use std::sync::Once;

static INIT: Once = Once::new();

pub struct DataRepository {
    active_board: Option<Board>,
    active_boardlist: Option<BoardList>,
    active_card: Option<Card>,
    active_checklist: Option<CardChecklist>,

    cache_boards: Option<Vec<Board>>,
    cache_boardlists: Option<Vec<BoardList>>,
    cache_cards: Option<Vec<Card>>,
    cache_checklists: Option<Vec<CardChecklist>>,
    cache_labels: Option<Vec<CardLabel>>,
}

impl DataRepository {
    pub async fn new(config: Option<serde_json::Value>) -> Option<DataRepository> {
        let mut dr: Option<DataRepository> = None;
        MongoDataStore::init(config.clone()).await;

        INIT.call_once(|| {
            TrelloDataStore::init(config); // TODO: Change this

            dr = Some(DataRepository {
                active_board: None,
                active_boardlist: None,
                active_card: None,
                active_checklist: None,
                cache_boards: None,
                cache_boardlists: None,
                cache_cards: None,
                cache_checklists: None,
                cache_labels: None,
            });
        });

        dr
    }

    fn invalidate_caches(&mut self, boardlists: bool, cards: bool, checklists: bool, labels: bool) {
        if boardlists {
            self.cache_boardlists.take();
        }

        if cards {
            self.cache_cards.take();
        }

        if checklists {
            self.cache_checklists.take();
        }

        if labels {
            self.cache_labels.take();
        }
    }

    pub async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let trello_boards_result = TrelloDataStore::get_all_boards().await;

        match trello_boards_result {
            Ok(trello_boards) => {
                self.cache_boards = Some(trello_boards.clone());
                let synced_boards = MongoDataStore::sync_boards(trello_boards).await?;
                Ok(synced_boards)
            }

            Err(trello_why) => {
                let mongo_boards_result = MongoDataStore::get_all_boards().await;
                match mongo_boards_result {
                    Ok(mongo_boards) => {
                        self.cache_boards = Some(mongo_boards.clone());
                        Ok(mongo_boards)
                    }

                    Err(mongo_why) => {
                        if self.cache_boards.is_none() {
                            Err(trello_why)
                        } else {
                            Ok(self.cache_boards.clone().unwrap())
                        }
                    }
                }
            }
        }
    }

    pub async fn create_board(&mut self, name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let trello_board_result = TrelloDataStore::create_board(name).await;
        match trello_board_result {
            Ok(trello_board) => {
                if self.cache_boards.is_some() {
                    self.cache_boards
                        .as_mut()
                        .unwrap()
                        .push(trello_board.clone());
                }
                self.invalidate_caches(true, true, true, true);
                Ok(trello_board)
            }

            Err(trello_why) => Err(trello_why),
        }
    }

    pub async fn select_board(
        &mut self,
        name: &str,
    ) -> Result<Option<Board>, Box<dyn std::error::Error>> {
        let mut boards: Vec<Board> = vec![];
        if self.cache_boards.is_none() {
            let boards_result = self.get_all_boards().await;
            if let Ok(all_boards) = boards_result {
                boards = all_boards;
            }
        } else {
            boards = self.cache_boards.clone().unwrap();
        }

        let mut result_board: Option<Board> = None;
        for board in boards {
            if board.name.eq_ignore_ascii_case(name) {
                self.active_board.replace(board.clone());
                result_board.replace(board);
            }
        }

        self.invalidate_caches(true, true, true, true);
        Ok(result_board)
    }

    pub async fn get_all_board_labels(
        &mut self,
        board: Option<Board>,
    ) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        let board_id: ID;

        if board.is_none() {
            if self.cache_labels.is_none() {
                if self.active_board.is_none() {
                    return Err(Box::new(InvalidInputError { message: Some(String::from("No board has been selected. Unable to infer which board's labels to get"))}));
                } else {
                    board_id = self.active_board.clone().unwrap()._id;
                }
            } else {
                let labels = self.cache_labels.clone().unwrap();
                return Ok(labels);
            }
        } else {
            board_id = board.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
        }

        let labels_result = TrelloDataStore::get_all_board_labels(board_id).await;
        match labels_result {
            Ok(trello_labels) => {
                self.cache_labels.replace(trello_labels.clone());
                Ok(trello_labels)
            }

            Err(why) => {
                Err(why)
            }
        }
    }

    pub async fn delete_board_label(&mut self, board: Option<Board>, label_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let board_id: ID;
        let labels: Vec<CardLabel>;
        let mut label_id: Option<ID> = None;

        if board.is_none() && self.active_board.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No board has been selected. Unable to infer which board's label to delete"))}));
        } else {
            board_id = if board.is_some() {
                self.active_board.replace(board.unwrap().clone());
                self.active_board.clone().unwrap()._id
            } else {
                self.active_board.clone().unwrap()._id
            };
        }

        labels = if self.cache_labels.is_none() {
            self.get_all_board_labels(Some(self.active_board.clone().unwrap())).await?
        } else {
            self.cache_labels.clone().unwrap()
        };

        for label in labels {
            if label.name.eq_ignore_ascii_case(label_name) && label.board_id == board_id {
                label_id = Some(label._id);
                break;
            }
        }

        self.invalidate_caches(false, true, false, true);
        if label_id.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("Could not find the given label name for the chosen board"))}));
        } else {
            return TrelloDataStore::delete_board_label(label_id.unwrap()).await;
        }
    }

    pub async fn update_board_label(&mut self, board: Option<Board>, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let board_id: ID;
        let mut label_id: Option<ID> = None;
        let labels: Vec<CardLabel>;

        if board.is_none() && self.active_board.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No board has been selected. Unable to infer which board's label to delete"))}));
        } else {
            board_id = if board.is_some() {
                self.active_board.replace(board.unwrap().clone());
                self.active_board.clone().unwrap()._id
            } else {
                self.active_board.clone().unwrap()._id
            };
        }

        labels = if self.cache_labels.is_none() {
            self.get_all_board_labels(Some(self.active_board.clone().unwrap())).await?
        } else {
            self.cache_labels.clone().unwrap()
        };

        for label in labels {
            if (label.name.eq_ignore_ascii_case(name) || label.color.eq_ignore_ascii_case(color)) && label.board_id == board_id {
                label_id = Some(label._id);
                break;
            }
        }

        self.invalidate_caches(false, true, false, true);
        if label_id.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("Could not find the given label for the chosen board"))}));
        } else {
            return TrelloDataStore::update_board_label(label_id.unwrap(), name, color).await;
        }
    }

    pub async fn create_board_label(&mut self, board: Option<Board>, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let board_id: ID;

        if board.is_none() && self.active_board.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No board has been selected. Unable to infer which board's label to create"))}));
        } else {
            board_id = if board.is_some() {
                self.active_board.replace(board.unwrap().clone());
                self.active_board.clone().unwrap()._id
            } else {
                self.active_board.clone().unwrap()._id
            };
        }
        self.invalidate_caches(false, true, false, true);
        return TrelloDataStore::create_board_label(board_id, name, color).await;
    }

    pub async fn get_all_board_lists(
        &mut self,
        board: Option<Board>,
    ) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        let board_id: ID;

        if board.is_none() {
            if self.cache_boardlists.is_none() {
                if self.active_board.is_none() {
                    return Err(Box::new(InvalidInputError { message: Some(String::from("No board has been selected. Unable to infer which board's lists to get"))}));
                } else {
                    board_id = self.active_board.clone().unwrap()._id;
                }
            } else {
                let lists = self.cache_boardlists.clone().unwrap();
                return Ok(lists);
            }
        } else {
            board_id = board.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
        }

        let lists_result = TrelloDataStore::get_all_board_lists(board_id).await;
        match lists_result {
            Ok(trello_lists) => {
                self.cache_boardlists.replace(trello_lists.clone());
                Ok(trello_lists)
            }

            Err(why) => {
                Err(why)
            }
        }
    }

    pub async fn create_board_list(&mut self, board: Option<Board>, name: &str) -> Result<BoardList, Box<dyn std::error::Error>> {
        let board_id: ID;

        if board.is_none() && self.active_board.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No board has been selected. Unable to infer which board's list to create"))}));
        } else {
            board_id = if board.is_some() {
                self.active_board.replace(board.unwrap().clone());
                self.active_board.clone().unwrap()._id
            } else {
                self.active_board.clone().unwrap()._id
            };
        }
        self.invalidate_caches(false, true, true, true);
        return TrelloDataStore::create_board_list(board_id, name).await;
    }

    pub async fn select_board_list(
        &mut self,
        name: &str,
        board: Option<Board>
    ) -> Result<Option<BoardList>, Box<dyn std::error::Error>> {
        let mut lists: Vec<BoardList> = vec![];
        if self.cache_boardlists.is_none() {
            let lists_result = self.get_all_board_lists(board).await;
            if let Ok(all_lists) = lists_result {
                lists = all_lists;
            }
        } else {
            lists = self.cache_boardlists.clone().unwrap();
        }

        let mut result_list: Option<BoardList> = None;
        for list in lists {
            if list.name.eq_ignore_ascii_case(name) {
                self.active_boardlist.replace(list.clone());
                result_list.replace(list);
            }
        }

        self.invalidate_caches(true, true, true, true);
        Ok(result_list)
    }

    pub async fn get_all_list_cards(
        &mut self,
        list: Option<BoardList>,
    ) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let list_id: ID;

        if list.is_none() {
            if self.cache_cards.is_none() {
                if self.active_boardlist.is_none() {
                    return Err(Box::new(InvalidInputError { message: Some(String::from("No list has been selected. Unable to infer which list's cards to get"))}));
                } else {
                    list_id = self.active_boardlist.clone().unwrap()._id;
                }
            } else {
                let cards = self.cache_cards.clone().unwrap();
                return Ok(cards);
            }
        } else {
            list_id = list.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
        }

        let cards_result = TrelloDataStore::get_all_list_cards(list_id).await;
        match cards_result {
            Ok(trello_cards) => {
                self.cache_cards.replace(trello_cards.clone());
                Ok(trello_cards)
            }

            Err(why) => {
                Err(why)
            }
        }
    }

    pub async fn create_list_card(&mut self, list: Option<BoardList>, name: &str) -> Result<Card, Box<dyn std::error::Error>> {
        let list_id: ID;

        if list.is_none() && self.active_boardlist.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No list has been selected. Unable to infer which list's card to create"))}));
        } else {
            list_id = if list.is_some() {
                self.active_boardlist.replace(list.unwrap().clone());
                self.active_boardlist.clone().unwrap()._id
            } else {
                self.active_boardlist.clone().unwrap()._id
            };
        }
        self.invalidate_caches(false, true, true, false);
        TrelloDataStore::create_list_card(list_id, name).await
    }

    pub async fn select_list_card(
        &mut self,
        name: &str,
        list: Option<BoardList>
    ) -> Result<Option<Card>, Box<dyn std::error::Error>> {
        let mut cards: Vec<Card> = vec![];
        if self.cache_cards.is_none() {
            let cards_result = self.get_all_list_cards(list).await;
            if let Ok(all_cards) = cards_result {
                cards = all_cards;
            }
        } else {
            cards = self.cache_cards.clone().unwrap();
        }

        let mut result_card: Option<Card> = None;
        for card in cards {
            if card.name.eq_ignore_ascii_case(name) {
                self.active_card.replace(card.clone());
                result_card.replace(card);
                break;
            }
        }

        self.invalidate_caches(true, true, true, true);
        Ok(result_card)
    }

    pub async fn update_card(&mut self, card: &Card) -> Result<Card, Box<dyn std::error::Error>> {
        self.invalidate_caches(false, true, false, false);
        TrelloDataStore::update_card(card).await
    }

    pub async fn get_card_comments(&mut self, card: Option<Card>) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {
        let card_id: ID;

        if card.is_none() {
            if self.active_card.is_none() {
                return Err(Box::new(InvalidInputError { message: Some(String::from("No card has been selected or provided. Unable to infer which card's comments to get"))}));
            } else {
                card_id = self.active_card.clone().unwrap()._id;
                return TrelloDataStore::get_card_comments(card_id).await;
            }
        } else {
            card_id = card.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
            return TrelloDataStore::get_card_comments(card_id).await;
        }
    }

    pub async fn add_card_comment(&mut self, card: Option<Card>, text: &str) -> Result<CardComment, Box<dyn std::error::Error>> {
        let card_id: ID;

        if card.is_none() {
            if self.active_card.is_none() {
                return Err(Box::new(InvalidInputError { message: Some(String::from("No card has been selected or provided. Unable to infer which card to add a comment to"))}));
            } else {
                card_id = self.active_card.clone().unwrap()._id;
                return TrelloDataStore::add_card_comment(card_id, text).await;
            }
        } else {
            card_id = card.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
            return TrelloDataStore::add_card_comment(card_id, text).await;
        }
    }

    pub async fn get_card_checklists(&mut self, card: Option<Card>) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>> {
        let card_id: ID;

        if card.is_none() {
            if self.cache_checklists.is_none() {
                if self.active_card.is_none() {
                    return Err(Box::new(InvalidInputError { message: Some(String::from("No card has been selected. Unable to infer which card's checklists to get"))}));
                } else {
                    card_id = self.active_card.clone().unwrap()._id;
                }
            } else {
                let checklists = self.cache_checklists.clone().unwrap();
                return Ok(checklists);
            }
        } else {
            card_id = card.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
        }

        let checklists_result = TrelloDataStore::get_card_checklists(card_id).await;
        match checklists_result {
            Ok(trello_checklists) => {
                self.cache_checklists.replace(trello_checklists.clone());
                Ok(trello_checklists)
            }

            Err(why) => {
                Err(why)
            }
        }
    }

    pub async fn create_card_checklist(&mut self, card: Option<Card>, name: &str) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        let card_id: ID;

        if card.is_none() && self.active_card.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No card has been selected. Unable to infer which card's checklist to create"))}));
        } else {
            card_id = if card.is_some() {
                self.active_card.replace(card.unwrap().clone());
                self.active_card.clone().unwrap()._id
            } else {
                self.active_card.clone().unwrap()._id
            };
        }
        self.invalidate_caches(false, false, true, false);
        TrelloDataStore::create_card_checklist(card_id, name).await
    }

    pub async fn select_card_checklist(&mut self, card: Option<Card>, name: &str) -> Result<Option<CardChecklist>, Box<dyn std::error::Error>> {
        let mut checklists: Vec<CardChecklist> = vec![];
        if self.cache_checklists.is_none() {
            let checklist_result = self.get_card_checklists(card).await;
            if let Ok(all_checklists) = checklist_result {
                checklists = all_checklists;
            }
        } else {
            checklists = self.cache_checklists.clone().unwrap();
        }

        let mut result_checklist: Option<CardChecklist> = None;
        for checklist in checklists {
            if checklist.name.eq_ignore_ascii_case(name) {
                self.active_checklist.replace(checklist.clone());
                result_checklist.replace(checklist);
                break;
            }
        }

        self.invalidate_caches(true, true, true, true);
        Ok(result_checklist)
    }

    pub async fn get_checklist_tasks(&mut self, checklist: Option<CardChecklist>) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>> {
        let checklist_id: ID;

        if checklist.is_none() {
            if self.active_checklist.is_none() {
                return Err(Box::new(InvalidInputError { message: Some(String::from("No Checklist has been selected. Unable to infer which checklist's tasks to get"))}));
            } else {
                checklist_id = self.active_checklist.clone().unwrap()._id;
            }
        } else {
            checklist_id = checklist.clone().unwrap()._id;
            self.invalidate_caches(true, true, true, true);
        }

        TrelloDataStore::get_checklist_tasks(checklist_id).await
    }

    pub async fn create_checklist_task(&mut self, checklist: Option<CardChecklist>, name: &str) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let checklist_id: ID;

        if checklist.is_none() && self.active_checklist.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No checklist has been selected. Unable to infer which checklist's task to create"))}));
        } else {
            checklist_id = if checklist.is_some() {
                self.active_checklist.replace(checklist.unwrap().clone());
                self.active_checklist.clone().unwrap()._id
            } else {
                self.active_checklist.clone().unwrap()._id
            };
        }
        TrelloDataStore::create_checklist_task(checklist_id, name).await
    }

    pub async fn update_checklist_task(&mut self, card: Option<Card>, task: CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let card_id: ID;

        if card.is_none() && self.active_card.is_none() {
            return Err(Box::new(InvalidInputError { message: Some(String::from("No card has been selected. Unable to infer which card's task to update"))}));
        } else {
            card_id = if card.is_some() {
                self.active_card.replace(card.unwrap().clone());
                self.active_card.clone().unwrap()._id
            } else {
                self.active_card.clone().unwrap()._id
            };
        }
        TrelloDataStore::update_checklist_task(card_id, &task).await
    }
}

#[async_trait]
pub trait DataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>>;
    async fn create_board(name: &str) -> Result<Board, Box<dyn std::error::Error>>;

    async fn get_all_board_labels(
        board_id: ID,
    ) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>>;
    async fn delete_board_label(label_id: ID) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_board_label(label_id: ID, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>>;
    async fn create_board_label(board_id: ID, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>>;

    async fn get_all_board_lists(board_id: ID) -> Result<Vec<BoardList>, Box<dyn std::error::Error>>;
    async fn create_board_list(board_id: ID, name: &str) -> Result<BoardList, Box<dyn std::error::Error>>;

    async fn get_all_list_cards(list_id: ID) -> Result<Vec<Card>, Box<dyn std::error::Error>>;
    async fn create_list_card(list_id: ID, name: &str) -> Result<Card, Box<dyn std::error::Error>>;
    async fn update_card(card: &Card) -> Result<Card, Box<dyn std::error::Error>>;
    async fn get_card_comments(card_id: ID) -> Result<Vec<CardComment>, Box<dyn std::error::Error>>;
    async fn add_card_comment(card_id: ID, text: &str) -> Result<CardComment, Box<dyn std::error::Error>>;

    async fn get_card_checklists(card_id: ID) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>>;
    async fn create_card_checklist(card_id: ID, name: &str) -> Result<CardChecklist, Box<dyn std::error::Error>>;
    async fn get_checklist_tasks(checklist_id: ID) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>>;
    async fn create_checklist_task(checklist_id: ID, name: &str) -> Result<CardChecklistTask, Box<dyn std::error::Error>>;
    async fn update_checklist_task(card_id: ID, task: &CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>>;
}
