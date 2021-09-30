use crate::control::command_executor::CommandExecutor;
use crate::control::*;
use crate::data::*;

use std::io::{self, Write};

struct InteractiveCli {
    command_exec: CommandExecutor,
}

impl InteractiveCli {
    fn print_prompt(
        &mut self,
        board: &Option<Board>,
        list: &Option<BoardList>,
        card: &Option<Card>,
        checklist: &Option<CardChecklist>,
    ) {
        let mut path = String::from("");
        if board.is_some() {
            path.push_str(&board.as_ref().unwrap().name);
            if list.is_some() {
                path.push('/');
                path.push_str(&list.as_ref().unwrap().name);
                if card.is_some() {
                    path.push('/');
                    path.push_str(&card.as_ref().unwrap().name);
                    if checklist.is_some() {
                        path.push('/');
                        path.push_str(&checklist.as_ref().unwrap().name);
                    }
                }
            }
        }

        path.push('>');
        print!("{}", path);
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
        let available_commands = vec!["get-all", "help"];

        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "get-all" => {
                let boards_result = self.command_exec.get_all_boards().await;

                match boards_result {
                    Ok(command_result) => {
                        println!("{}", command_result.result_string.unwrap());
                        match command_result.result_code {
                            CommandResultCode::Success => {
                                let boards: Vec<Board> = command_result.result.unwrap();
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

                    Err(why) => println!("Failed to get all boards: {}", why),
                };
            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            },
        }
    }
}

pub async fn run() {
    let mut current_board: Option<Board> = None;
    let mut current_list: Option<BoardList> = None;
    let mut current_card: Option<Card> = None;
    let mut current_checklist: Option<CardChecklist> = None;

    let command_exec = CommandExecutor::new().await;
    let mut cli = InteractiveCli {
        command_exec: command_exec,
    };

    let available_commands = vec!["board", "exit", "help"];

    loop {
        cli.print_prompt(
            &current_board,
            &current_list,
            &current_card,
            &current_checklist,
        );
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
            },
        }
    }
}
