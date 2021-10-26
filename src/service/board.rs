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

    pub async fn get_all_board_labels(&mut self, board: Option<Board>) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        self.data_repo.get_all_board_labels(board).await
    }

    pub async fn delete_board_label(&mut self, board: Option<Board>, label_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.data_repo.delete_board_label(board, label_name).await
    }

    pub async fn update_board_label(&mut self, board: Option<Board>, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        self.data_repo.update_board_label(board, name, color).await
    }

    pub async fn create_board_label(&mut self, board: Option<Board>, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        self.data_repo.create_board_label(board, name, color).await
    }

    pub async fn get_all_board_lists(&mut self, board: Option<Board>) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        self.data_repo.get_all_board_lists(board).await
    }

    pub async fn create_board_list(&mut self, board: Option<Board>, name: &str) -> Result<BoardList, Box<dyn std::error::Error>> {
        self.data_repo.create_board_list(board, name).await
    }

    pub async fn select_board_list(&mut self, name: &str, board: Option<Board>) -> Result<Option<BoardList>, Box<dyn std::error::Error>> {
        self.data_repo.select_board_list(name, board).await
    }

    pub async fn get_all_list_cards(&mut self, list: Option<BoardList>) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        self.data_repo.get_all_list_cards(list).await
    }

    pub async fn create_list_card(&mut self, list: Option<BoardList>, name: &str) -> Result<Card, Box<dyn std::error::Error>> {
        self.data_repo.create_list_card(list, name).await
    }

    pub async fn select_list_card(&mut self, name: &str, list: Option<BoardList>) -> Result<Option<Card>, Box<dyn std::error::Error>> {
        self.data_repo.select_list_card(name, list).await
    }

    pub async fn update_card(&mut self, card: &Card) -> Result<Card, Box<dyn std::error::Error>> {
        self.data_repo.update_card(card).await
    }

    pub async fn get_card_comments(&mut self, card: Option<Card>) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {
        self.data_repo.get_card_comments(card).await
    }

    pub async fn add_card_comment(&mut self, card: Option<Card>, text: &str) -> Result<CardComment, Box<dyn std::error::Error>> {
        self.data_repo.add_card_comment(card, text).await
    }

    pub async fn get_card_checklists(&mut self, card: Option<Card>) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>> {
        self.data_repo.get_card_checklists(card).await
    }

    pub async fn create_card_checklist(&mut self, card: Option<Card>, name: &str) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        self.data_repo.create_card_checklist(card, name).await
    }

    pub async fn select_card_checklist(&mut self, card: Option<Card>, name: &str) -> Result<Option<CardChecklist>, Box<dyn std::error::Error>> {
        self.data_repo.select_card_checklist(card, name).await
    }

    pub async fn get_checklist_tasks(&mut self, checklist: Option<CardChecklist>) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>> {
        self.data_repo.get_checklist_tasks(checklist).await
    }

    pub async fn create_checklist_task(&mut self, checklist: Option<CardChecklist>, name: &str) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        self.data_repo.create_checklist_task(checklist, name).await
    }

    pub async fn update_checklist_task(&mut self, card: Option<Card>, task: CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        self.data_repo.update_checklist_task(card, task).await
    }
}
