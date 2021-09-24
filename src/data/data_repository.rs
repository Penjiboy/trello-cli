use crate::data::*;
use crate::data::trello_data_store::TrelloDataStore;

use async_trait::async_trait;

pub struct DataRepository;

#[async_trait]
pub trait DataStore {
    fn init();
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>>;
}