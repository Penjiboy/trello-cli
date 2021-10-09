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

    pub async fn select_board(&mut self, name: &str) -> Result<Option<Board>, Box<dyn std::error::Error>> {
        self.data_repo.select_board(name).await
    }

    pub async fn create_board(&mut self, name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        self.data_repo.create_board(name).await
    }
}
