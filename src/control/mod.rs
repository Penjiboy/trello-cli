use crate::service::board as BoardService;

pub mod interactive_cli;

pub enum CommandResultCode {
    Success,
    Failed,
}

pub struct CommandResult<T> {
    pub result_code: CommandResultCode,
    pub result: Option<T>,
    pub result_string: Option<String>,
}

pub mod command_executor {
    use crate::control::*;
    use crate::data::*;
    use crate::service::board::BoardService;

    pub struct CommandExecutor {
        board_service: BoardService,
    }

    impl CommandExecutor {
        pub async fn new() -> CommandExecutor {
            let bs = BoardService::new().await;
            CommandExecutor { board_service: bs }
        }

        pub async fn get_all_boards(
            &mut self,
        ) -> CommandResult<Vec<Board>> {
            let boards_result = self.board_service.get_all_boards().await;
            let command_result: CommandResult<Vec<Board>> = match boards_result {
                Ok(boards) => {
                    let result_code = CommandResultCode::Success;
                    let result_string = format!("Retrieved {} boards", boards.len());
                    CommandResult {
                        result_code: result_code,
                        result: Some(boards),
                        result_string: Some(result_string),
                    }
                }

                Err(why) => {
                    let result_code = CommandResultCode::Failed;
                    let result_string = String::from(why.to_string());
                    CommandResult {
                        result_code: result_code,
                        result: None,
                        result_string: Some(result_string),
                    }
                }
            };

            command_result
        }

        pub async fn select_board(&mut self, name: &str) -> CommandResult<Board> {
            let board_result = self.board_service.select_board(name).await;
            let command_result: CommandResult<Board> = match board_result {
                Ok(board) => {
                    if board.is_some() {
                        let _board = board.unwrap();
                        let res_string = format!("Selected board {}", _board.name);
                        CommandResult {
                            result_code: CommandResultCode::Success,
                            result: Some(_board),
                            result_string: Some(res_string)
                        }
                    } else {
                        CommandResult {
                            result_code: CommandResultCode::Failed,
                            result: None,
                            result_string: Some(String::from("Failed to select board"))
                        }
                    }
                }

                Err(why) => {
                    CommandResult {
                        result_code: CommandResultCode::Failed,
                        result: None,
                        result_string: Some(String::from(why.to_string()))
                    }
                }
            };

            command_result
        }
    }
}
