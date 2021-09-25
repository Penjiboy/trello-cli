use crate::data::data_repository::*;
use crate::data::*;

pub fn init() {
    DataRepository::init();
}
pub async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
    DataRepository::get_all_boards().await
}