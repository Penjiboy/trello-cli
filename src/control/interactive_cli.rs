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
                if let CommandResultCode::Success = board_result.result_code {
                    self.current_board.replace(board_result.result.unwrap());
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

    let available_commands = vec!["board", "exit", "help"];

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

            _ => {
                cli.print_invalid_command(None);
                cli.print_available_commands(&available_commands);
            }
        }
    }
}
