use crate::data::data_repository::*;
use crate::data::*;

pub struct BoardService {
    data_repo: DataRepository,
}

impl BoardService {
    pub async fn new() -> BoardService {
        let dr = DataRepository::new();
        BoardService { data_repo: dr.await.unwrap() }
    }

    pub async fn get_all_boards(&mut self) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        self.data_repo.get_all_boards().await
    }
}
