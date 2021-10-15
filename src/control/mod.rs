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

        pub async fn get_all_boards(&mut self) -> CommandResult<Vec<Board>> {
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
                            result_string: Some(res_string),
                        }
                    } else {
                        CommandResult {
                            result_code: CommandResultCode::Failed,
                            result: None,
                            result_string: Some(String::from("Failed to select board")),
                        }
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string())),
                },
            };

            command_result
        }

        pub async fn create_board(&mut self, name: &str) -> CommandResult<Board> {
            let board_result = self.board_service.create_board(name).await;
            let command_result: CommandResult<Board> = match board_result {
                Ok(board) => {
                    let res_string = format!("Created board {}", board.name);
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(board),
                        result_string: Some(res_string),
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string())),
                },
            };

            command_result
        }

        pub async fn get_all_board_labels(&mut self, board: Option<Board>) -> CommandResult<Vec<CardLabel>> {
            let labels_result = self.board_service.get_all_board_labels(board).await;
            let command_result: CommandResult<Vec<CardLabel>> = match labels_result {
                Ok(labels) => {
                    let res_string = format!("Retrieved {} board labels", labels.len());
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(labels),
                        result_string: Some(res_string)
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string()))
                }
            };

            command_result
        }

        pub async fn delete_board_label(&mut self, board: Option<Board>, label_name: &str) -> CommandResult<()> {
            let delete_result = self.board_service.delete_board_label(board, label_name).await;
            let command_result: CommandResult<()> = match delete_result {
                Ok(()) => {
                    let res_string = format!("Deleted label");
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(()),
                        result_string: Some(res_string)
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string()))
                }
            };

            command_result
        }

        pub async fn update_board_label(&mut self, board: Option<Board>, name: &str, color: &str) -> CommandResult<CardLabel> {
            let update_result = self.board_service.update_board_label(board, name, color).await;
            let command_result: CommandResult<CardLabel> = match update_result {
                Ok(label) => {
                    let res_string = format!("Updated label");
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(label),
                        result_string: Some(res_string)
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string()))
                }
            };

            command_result
        }

        pub async fn create_board_label(&mut self, board: Option<Board>, name: &str, color: &str) -> CommandResult<CardLabel> {
            let create_result = self.board_service.create_board_label(board, name, color).await;
            let command_result: CommandResult<CardLabel> = match create_result {
                Ok(label) => {
                    let res_string = format!("Created label");
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(label),
                        result_string: Some(res_string)
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string()))
                }
            };

            command_result
        }

        pub async fn get_all_board_lists(&mut self, board: Option<Board>) -> CommandResult<Vec<BoardList>> {
            let lists_result = self.board_service.get_all_board_lists(board).await;
            let command_result: CommandResult<Vec<BoardList>> = match lists_result {
                Ok(lists) => {
                    let res_string = format!("Retrieved {} lists", lists.len());
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(lists),
                        result_string: Some(res_string)
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string()))
                }
            };

            command_result
        }

        pub async fn create_board_list(&mut self, board: Option<Board>, name: &str) -> CommandResult<BoardList> {
            let create_result = self.board_service.create_board_list(board, name).await;
            let command_result: CommandResult<BoardList> = match create_result {
                Ok(list) => {
                    let res_string = format!("Created list");
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(list),
                        result_string: Some(res_string)
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string()))
                }
            };

            command_result
        }

        pub async fn select_board_list(&mut self, name: &str, board: Option<Board>) -> CommandResult<BoardList> {
            let list_result = self.board_service.select_board_list(name, board).await;
            let command_result: CommandResult<BoardList> = match list_result {
                Ok(list) => {
                    if list.is_some() {
                        let _list = list.unwrap();
                        let res_string = format!("Selected list {}", _list.name);
                        CommandResult {
                            result_code: CommandResultCode::Success,
                            result: Some(_list),
                            result_string: Some(res_string),
                        }
                    } else {
                        CommandResult {
                            result_code: CommandResultCode::Failed,
                            result: None,
                            result_string: Some(String::from("Failed to select list")),
                        }
                    }
                }

                Err(why) => CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(String::from(why.to_string())),
                },
            };

            command_result
        }
    }
}
