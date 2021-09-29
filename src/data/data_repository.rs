use crate::data::trello_data_store::TrelloDataStore;
use crate::data::*;

use async_trait::async_trait;

use std::sync::Once;

static INIT: Once = Once::new();

pub struct DataRepository {
    trello_store: TrelloDataStore,
    active_board: Option<Board>,
    active_boardlist: Option<BoardList>,
    active_card: Option<Card>,
    active_checklist: Option<CardChecklist>,

    cache_boards: Option<Vec<Board>>,
    cache_boardlists: Option<Vec<Board>>,
    cache_cards: Option<Vec<Card>>,
    cache_checklists: Option<Vec<CardChecklist>>,
}

impl DataRepository {
    pub fn new() -> Option<DataRepository> {
        let mut dr: Option<DataRepository> = None;
        INIT.call_once(|| {
            let trello = TrelloDataStore::new(None, None); // TODO: Change this
            dr = Some(DataRepository {
                trello_store: trello,
                active_board: None,
                active_boardlist: None,
                active_card: None,
                active_checklist: None,
                cache_boards: None,
                cache_boardlists: None,
                cache_cards: None,
                cache_checklists: None,
            });
        });

        dr
    }
}

#[async_trait]
pub trait DataStore {
    async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl DataStore for DataRepository {
    async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let boards_result: Result<Vec<Board>, Box<dyn std::error::Error>> =
            self.trello_store.get_all_boards().await;
        match boards_result {
            Ok(boards) => {
                self.cache_boards = Some(boards.clone());
                Ok(boards)
            }

            Err(why) => {
                if self.cache_boards.is_none() {
                    Err(why)
                } else {
                    Ok(self.cache_boards.clone().unwrap())
                }
            }
        }
    }
}
