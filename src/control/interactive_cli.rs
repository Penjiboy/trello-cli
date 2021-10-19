use crate::control::command_executor::CommandExecutor;
use crate::control::*;
use crate::data::*;

use std::io::{self, Write};

struct InteractiveCli {
    command_exec: CommandExecutor,

    current_board: Option<Board>,
    current_list: Option<BoardList>,
    current_card: Option<Card>,
    current_checklist: Option<CardChecklist>,
}

impl InteractiveCli {
    fn print_prompt(&mut self) {
        let mut path = String::from("");
        if self.current_board.is_some() {
            path.push_str(self.current_board.clone().unwrap().name.as_str());
            if self.current_list.is_some() {
                path.push('/');
                path.push_str(self.current_list.clone().unwrap().name.as_str());
                if self.current_card.is_some() {
                    path.push('/');
                    path.push_str(self.current_card.clone().unwrap().name.as_str());
                    if self.current_checklist.is_some() {
                        path.push('/');
                        path.push_str(self.current_checklist.clone().unwrap().name.as_str());
                    }
                }
            }
        }

        path.push('>');
        print!("\n{}", path);
        io::stdout().flush().unwrap();
    }

    fn print_invalid_command(&mut self, help: Option<String>) {
        println!("Invalid command. {}", help.unwrap_or(String::from("")));
    }

    fn print_available_commands(&mut self, commands: &Vec<&str>) {
        println!("Available commands: ");
        for command in commands {
            println!("  {}", command);
        }
    }

    async fn handle_label_command(&mut self, mut input_iter: std::str::SplitAsciiWhitespace<'_>) {
        let available_commands = vec![
            "get-all",
            "create <Label_Name> <Label_Color>",
            "delete <Label_Name>",
            "update <Label_Name> <Label_Color>",
            "help",
        ];
        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "get-all" => {
                let labels_result = self.command_exec.get_all_board_labels(None).await;
                println!("{}", labels_result.result_string.unwrap());
                match labels_result.result_code {
                    CommandResultCode::Success => {
                        let labels: Vec<CardLabel> = labels_result.result.unwrap();
                        println!("Labels: ");
                        for label in labels {
                            println!("  {name}: {color}", name = label.name, color = label.color);
                        }
                    }

                    CommandResultCode::Failed => {
                        println!("Command Failed. Do you have a board selected?");
                    }
                }
            }

            "create" => {
                let name = input_iter.next().unwrap_or("");
                let color = input_iter.next().unwrap_or("");
                if name.is_empty() || color.is_empty() {
                    self.print_invalid_command(Some(String::from(
                        "You must provide both name and color",
                    )));
                    self.print_available_commands(&available_commands);
                } else {
                    let create_result = self
                        .command_exec
                        .create_board_label(None, name, color)
                        .await;
                    println!("{}", create_result.result_string.unwrap());
                }
            }

            "delete" => {
                let remainder: Vec<&str> = input_iter.collect();
                let label_name = remainder.join(" ");
                let delete_result = self
                    .command_exec
                    .delete_board_label(None, &label_name)
                    .await;
                println!("{}", delete_result.result_string.unwrap());
            }

            "update" => {
                let name = input_iter.next().unwrap_or("");
                let color = input_iter.next().unwrap_or("");
                if name.is_empty() || color.is_empty() {
                    self.print_invalid_command(Some(String::from(
                        "You must provide both name and color",
                    )));
                    self.print_available_commands(&available_commands);
                } else {
                    let update_result = self
                        .command_exec
                        .update_board_label(None, name, color)
                        .await;
                    println!("{}", update_result.result_string.unwrap());
                }
            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            }
        }
    }

    async fn handle_board_command(&mut self, mut input_iter: std::str::SplitAsciiWhitespace<'_>) {
        let available_commands = vec!["get-all", "select <Name>", "create-new <Name>", "help"];
        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "get-all" => {
                let boards_result = self.command_exec.get_all_boards().await;
                println!("{}", boards_result.result_string.unwrap());
                match boards_result.result_code {
                    CommandResultCode::Success => {
                        let boards: Vec<Board> = boards_result.result.unwrap();
                        println!("Board Names: ");
                        for board in boards {
                            println!("  - {}", board.name);
                        }
                    }

                    CommandResultCode::Failed => {
                        println!("Command Failed.")
                    }
                };
            }

            "select" => {
                let remainder: Vec<&str> = input_iter.collect();
                let board_name = remainder.join(" ");
                let board_result = self.command_exec.select_board(&board_name).await;
                println!("{}", board_result.result_string.unwrap());
                if let CommandResultCode::Success = board_result.result_code {
                    self.current_board.replace(board_result.result.unwrap());
                }
            }

            "create-new" => {
                let remainder: Vec<&str> = input_iter.collect();
                let board_name = remainder.join(" ");
                let board_result = self.command_exec.create_board(&board_name).await;
                println!("{}", board_result.result_string.unwrap());
            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            }
        }
    }

    async fn handle_list_command(&mut self, mut input_iter: std::str::SplitAsciiWhitespace<'_>) {
        let available_commands = vec!["get-all", "select <Name>", "create <Name>", "get-cards", "create-card <Name>", "select-card <Name>", "help"];
        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "get-all" => {
                let lists_result = self.command_exec.get_all_board_lists(None).await;
                println!("{}", lists_result.result_string.unwrap());
                match lists_result.result_code {
                    CommandResultCode::Success => {
                        let lists: Vec<BoardList> = lists_result.result.unwrap();
                        println!("Lists: ");
                        for list in lists {
                            println!("  - {name}", name = list.name);
                        }
                    }

                    CommandResultCode::Failed => {
                        println!("Command Failed. Do you have a board selected?");
                    }
                }
            }

            "select" => {
                let remainder: Vec<&str> = input_iter.collect();
                let list_name = remainder.join(" ");
                if list_name.is_empty() {
                    self.print_invalid_command(Some(String::from("You must provide a name")));
                    self.print_available_commands(&available_commands);
                } else {
                    let list_result = self.command_exec.select_board_list(&list_name, None).await;
                    println!("{}", list_result.result_string.unwrap());
                    if let CommandResultCode::Success = list_result.result_code {
                        self.current_list.replace(list_result.result.unwrap());
                    }
                }
            }

            "create" => {
                let remainder: Vec<&str> = input_iter.collect();
                let list_name = remainder.join(" ");
                if list_name.is_empty() {
                    self.print_invalid_command(Some(String::from("You must provide a name")));
                    self.print_available_commands(&available_commands);
                } else {
                    let list_result = self.command_exec.create_board_list(None, &list_name).await;
                    println!("{}", list_result.result_string.unwrap());
                }
            }

            "get-cards" => {
                let cards_result = self.command_exec.get_all_list_cards(None).await;
                println!("{}", cards_result.result_string.unwrap());
                if let CommandResultCode::Success = cards_result.result_code {
                    println!("Cards:");
                    for card in cards_result.result.unwrap() {
                        println!("  - {}", card.name);
                    }
                }

            }

            "create-card" => {
                let remainder: Vec<&str> = input_iter.collect();
                let card_name = remainder.join(" ");
                if card_name.is_empty() {
                    self.print_invalid_command(Some(String::from("You must provide a name")));
                    self.print_available_commands(&available_commands);
                } else {
                    let card_result = self.command_exec.create_list_card(None, &card_name).await;
                    println!("{}", card_result.result_string.unwrap());
                }
            }

            "select-card" => {
                let remainder: Vec<&str> = input_iter.collect();
                let card_name = remainder.join(" ");
                if card_name.is_empty() {
                    self.print_invalid_command(Some(String::from("You must provide a name")));
                    self.print_available_commands(&available_commands);
                } else {
                    let card_result = self.command_exec.select_list_card(&card_name, None).await;
                    println!("{}", card_result.result_string.unwrap());
                    if let CommandResultCode::Success = card_result.result_code {
                        self.current_card.replace(card_result.result.unwrap());
                        let description: String = self.current_card.clone().unwrap().description;
                        println!("Card Description: \n{}", description);
                    }
                }
            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            }
        }
    }

    async fn handle_card_command(&mut self, mut input_iter: std::str::SplitAsciiWhitespace<'_>) {
        let available_commands = vec!["get-description", "edit-description", "move-to-list <ListName>", "help"];
        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "get-description" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card: Card = self.current_card.clone().unwrap();
                    println!("Card Description:\n{}", card.description);
                }
            }

            "edit-description" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let mut card: Card = self.current_card.clone().unwrap();
                    let remainder: Vec<&str> = input_iter.collect();
                    let description = remainder.join(" ");
                    if description.is_empty() {
                        self.print_invalid_command(Some(String::from("You must provide a description")));
                        self.print_available_commands(&available_commands);
                    } else {
                        card.description = description;
                        let card_result = self.command_exec.update_card(&card).await;
                        println!("{}", card_result.result_string.unwrap());
                        if let CommandResultCode::Success = card_result.result_code {
                            self.current_card.replace(card_result.result.unwrap());
                        }
                    }
                }
            }

            "move-to-list" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card: Card = self.current_card.clone().unwrap();
                    let remainder: Vec<&str> = input_iter.collect();
                    let list_name = remainder.join(" ");
                    if list_name.is_empty() {
                        self.print_invalid_command(Some(String::from("You must provide a list name")));
                        self.print_available_commands(&available_commands);
                    } else {
                        let card_result = self.command_exec.move_card_to_list(card, &list_name).await;
                        println!("{}", card_result.result_string.unwrap());
                        if let CommandResultCode::Success = card_result.result_code {
                            self.current_card.replace(card_result.result.unwrap());
                            let list_result = self.command_exec.select_board_list(&list_name, None).await;
                            self.current_list.replace(list_result.result.unwrap());
                        }
                    }
                }

            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            }

        }

    }
}

pub async fn run() {
    let command_exec = CommandExecutor::new().await;
    let mut cli = InteractiveCli {
        command_exec: command_exec,
        current_board: None,
        current_list: None,
        current_card: None,
        current_checklist: None,
    };

    let available_commands = vec!["board", "label", "list", "card", "exit", "help"];

    loop {
        cli.print_prompt();

        let mut input = String::new();
        let read_result = io::stdin().read_line(&mut input);
        if read_result.is_err() {
            println!("Error while reading input: {}", read_result.unwrap_err());
            break;
        }

        if input.trim_end().eq_ignore_ascii_case("exit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let mut input_iter = input.split_ascii_whitespace();
        match input_iter.next().unwrap_or("") {
            "" => continue,
            "help" => cli.print_available_commands(&available_commands),

            "board" => {
                cli.handle_board_command(input_iter).await;
            }

            "label" => {
                cli.handle_label_command(input_iter).await;
            }

            "list" => {
                cli.handle_list_command(input_iter).await;
            }

            "card" => {
                cli.handle_card_command(input_iter).await;
            }
            _ => {
                cli.print_invalid_command(None);
                cli.print_available_commands(&available_commands);
            }
        }
    }
}
