use crate::data::mongo_data_store::MongoDataStore;
use crate::data::trello_data_store::TrelloDataStore;
use crate::data::*;

use async_trait::async_trait;

use std::sync::mpsc;
use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();

pub struct DataRepository {
    active_board: Option<Board>,
    active_boardlist: Option<BoardList>,
    active_card: Option<Card>,
    active_checklist: Option<CardChecklist>,

    cache_boards: Option<Vec<Board>>,
    cache_boardlists: Option<Vec<Board>>,
    cache_cards: Option<Vec<Card>>,
    cache_checklists: Option<Vec<CardChecklist>>,
    cache_labels: Option<Vec<CardLabel>>,
}

impl DataRepository {
    pub async fn new() -> Option<DataRepository> {
        let mut dr: Option<DataRepository> = None;
        MongoDataStore::init().await;

        INIT.call_once(|| {
            TrelloDataStore::init(None, None); // TODO: Change this

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
                Ok(trello_boards)
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
        let board_id: String;

        if board.is_none() {
            if self.cache_labels.is_none() {
                if self.active_board.is_none() {
                    return Err(Box::new(InvalidInputError {}));
                } else {
                    board_id = self.active_board.clone().unwrap().id.trello_id.unwrap();
                }
            } else {
                let labels = self.cache_labels.clone().unwrap();
                return Ok(labels);
            }
        } else {
            board_id = board.clone().unwrap().id.trello_id.unwrap();
            self.invalidate_caches(true, true, true, true);
        }

        let labels_result = TrelloDataStore::get_all_board_labels(&board_id).await;
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
}

#[async_trait]
pub trait DataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>>;
    async fn create_board(name: &str) -> Result<Board, Box<dyn std::error::Error>>;
    async fn get_all_board_labels(
        board_id: &str,
    ) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>>;
}
