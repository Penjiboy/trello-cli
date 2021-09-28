use crate::data::trello_data_store::TrelloDataStore;
use crate::data::*;

use async_trait::async_trait;

use std::sync::Once;

static mut data_repo: Option<DataRepository> = None;
static INIT: Once = Once::new();

pub struct DataRepository {
    trello_store: TrelloDataStore,
}

impl DataRepository {
    pub fn new() -> Option<DataRepository> {
        INIT.call_once(|| {
            let trello = TrelloDataStore::new(None, None); // TODO: Change this
            unsafe {
                data_repo = Some(DataRepository {
                    trello_store: trello,
                });
            }
        });

        unsafe {
            data_repo
        }
    }
}

#[async_trait]
pub trait DataStore {
    async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl DataStore for DataRepository {
    async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        self.trello_store.get_all_boards().await
    }
}
