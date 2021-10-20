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

        pub async fn get_all_list_cards(&mut self, list: Option<BoardList>) -> CommandResult<Vec<Card>> {
            let cards_result = self.board_service.get_all_list_cards(list).await;
            let command_result: CommandResult<Vec<Card>> = match cards_result {
                Ok(cards) => {
                    let res_string = format!("Retrieved {} cards", cards.len());
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(cards),
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

        pub async fn create_list_card(&mut self, list: Option<BoardList>, name: &str) -> CommandResult<Card> {
            let card_result = self.board_service.create_list_card(list, name).await;
            let command_result: CommandResult<Card> = match card_result {
                Ok(card) => {
                    let res_string = format!("Created card");
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(card),
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

        pub async fn select_list_card(&mut self, name: &str, list: Option<BoardList>) -> CommandResult<Card> {
            let card_result = self.board_service.select_list_card(name, list).await;
            let command_result: CommandResult<Card> = match card_result {
                Ok(card) => {
                    if card.is_some() {
                        let _card = card.unwrap();
                        let res_string = format!("Selected card {}", _card.name);
                        CommandResult {
                            result_code: CommandResultCode::Success,
                            result: Some(_card),
                            result_string: Some(res_string),
                        }
                    } else {
                        CommandResult {
                            result_code: CommandResultCode::Failed,
                            result: None,
                            result_string: Some(String::from("Failed to select card")),
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

        pub async fn update_card(&mut self, card: &Card) -> CommandResult<Card> {
            let card_result = self.board_service.update_card(card).await;
            let command_result: CommandResult<Card> = match card_result {
                Ok(card) => {
                    let res_string = format!("Updated card");
                    CommandResult {
                        result_code: CommandResultCode::Success,
                        result: Some(card),
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

        pub async fn move_card_to_list(&mut self, mut card: Card, list_name: &str) -> CommandResult<Card> {
            // Get the list (more importantly the list ID)
            let list_result: CommandResult<Vec<BoardList>> = self.get_all_board_lists(None).await;
            let mut list_id: Option<ID> = None;
            if let CommandResultCode::Success = list_result.result_code {
               let lists: Vec<BoardList> = list_result.result.unwrap();
               for list in lists {
                   if list.name.eq_ignore_ascii_case(list_name) {
                       list_id = Some(list.id);
                       break;
                   }
               }
            } 

            if list_id.is_none() {
                return CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(format!("Failed to get the target list"))
                };
            } else {
                // Update the card
                card.list_id = list_id.unwrap();
                return self.update_card(&card).await;
            }
        }

        pub async fn get_card_labels(&mut self, card: &Card) -> CommandResult<Vec<CardLabel>> {
            let all_labels_result: CommandResult<Vec<CardLabel>> = self.get_all_board_labels(None).await;
            if let CommandResultCode::Success = all_labels_result.result_code {
                let mut all_labels: Vec<CardLabel> = all_labels_result.result.unwrap();
                all_labels.retain(|label| card.label_ids.contains(&label.id));
                return CommandResult {
                    result_code: CommandResultCode::Success,
                    result: Some(all_labels),
                    result_string: Some(format!("Retrieved card label(s)"))
                }
            } else {
                return CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(format!("Failed to get card labels"))
                }
            }
        }

        pub async fn add_card_label(&mut self, mut card: Card, label_name: &str) -> CommandResult<Card> {
            let labels_result = self.get_all_board_labels(None).await;
            if let CommandResultCode::Success = labels_result.result_code {
                let card_labels: Vec<CardLabel> = labels_result.result.unwrap();
                let mut chosen_label: Option<CardLabel> = None;
                for label in card_labels {
                    if label.name.eq_ignore_ascii_case(label_name) {
                        chosen_label = Some(label);
                        break;
                    }
                }

                if chosen_label.is_some() {
                    card.label_ids.push(chosen_label.unwrap().id);
                    return self.update_card(&card).await;
                } else {
                    return CommandResult {
                        result_code: CommandResultCode::Failed,
                        result: None,
                        result_string: Some(format!("Could not find a board label with the given name"))
                    };
                }
            } else {
                return CommandResult {
                    result_code: CommandResultCode::Failed,
                    result: None,
                    result_string: Some(format!("Failed to get board labels"))
                };
            }
        }
    }
}
