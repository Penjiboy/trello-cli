use crate::data::mongo_data_store::MongoDataStore;
use crate::data::trello_data_store::TrelloDataStore;
use crate::data::*;

use async_trait::async_trait;

use std::sync::Once;
use std::thread;
use std::sync::mpsc;

static INIT: Once = Once::new();

pub struct DataRepository {
    trello_store: TrelloDataStore,
    mongo_store: MongoDataStore,

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
    pub async fn new() -> Option<DataRepository> {
        let mut dr: Option<DataRepository> = None;
        let mongo = MongoDataStore::new().await;
        INIT.call_once(|| {
            let trello = TrelloDataStore::new(None, None); // TODO: Change this
            dr = Some(DataRepository {
                trello_store: trello,
                mongo_store: mongo,

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
        let (tx, rx) = mpsc::channel::<Option<Vec<Board>>>();

        let handle = thread::spawn(move || {
            // async {
                // let mongo_boards_result = self.mongo_store.get_all_boards().await;
            // }
            // let active = self.active_board;
        });

        handle.join().unwrap();

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
