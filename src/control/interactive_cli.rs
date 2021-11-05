use crate::control::command_executor::CommandExecutor;
use crate::control::*;
use crate::data::*;

use std::io::{self, Write};
use std::convert::TryInto;

use chrono::{DateTime, TimeZone, Local};

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

    fn get_selection_from_prompt(&mut self, names: &Vec<&str>) -> usize {
        println!("Enter a number to choose one of the following options: ");
        let mut count: i8 = 0;
        for name in names {
            println!("  ({counter})  {name}", counter = count, name = name);
            count += 1;
        }

        println!("Your selection (default = 0): ");
        let mut input = String::new();
        let read_result = io::stdin().read_line(&mut input);
        if read_result.is_err() {
            println!("Error while reading input: {}", read_result.unwrap_err());
            return 0;
        } else if input.is_empty() {
            return 0;
        } else {
            let selection = input.trim().parse::<i32>();
            if selection.is_err() {
                println!("Invalid input");
                return 0;
            } 
            
            let mut index: usize = selection.unwrap().try_into().unwrap_or(0);
            if index > names.len() {
                index = 0;
            }
            return index;
        }
    }

    async fn handle_label_command(&mut self, mut input_iter: std::str::SplitAsciiWhitespace<'_>) {
        let available_commands = vec![
            "get-all",
            "create <Label_Name> <Label_Color>",
            "delete [<Label_Name>]",
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
                let mut label_name = remainder.join(" ");
                if label_name.is_empty() {
                    let labels: Vec<CardLabel> = self.command_exec.get_all_board_labels(None).await.result.unwrap_or(vec![]);
                    if labels.is_empty() {
                        println!("Found no labels to select");
                        return;
                    } else {
                        let label_names: Vec<&str> = labels.iter().map(|label| label.name.as_str()).collect::<Vec<_>>();
                        let index: usize = self.get_selection_from_prompt(&label_names);
                        label_name = label_names.get(index).unwrap_or(&"").to_string();
                    }
                }
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
        let available_commands = vec!["get-all", "select [<Name>]", "create-new <Name>", "help"];
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
                let mut board_name = remainder.join(" ");
                if board_name.is_empty() {
                    let boards: Vec<Board> = self.command_exec.get_all_boards().await.result.unwrap_or(vec![]);
                    if boards.is_empty() {
                        println!("Found no boards to select");
                        return;
                    } else {
                        let board_names: Vec<&str> = boards.iter().map(|board| board.name.as_str()).collect::<Vec<_>>();
                        let index: usize = self.get_selection_from_prompt(&board_names);
                        board_name = board_names.get(index).unwrap_or(&"").to_string();
                    }
                }
                let board_result = self.command_exec.select_board(&board_name).await;
                println!("{}", board_result.result_string.unwrap());
                if let CommandResultCode::Success = board_result.result_code {
                    self.current_board.replace(board_result.result.unwrap());
                    self.current_list.take();
                    self.current_card.take();
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
        let available_commands = vec!["get-all", "select [<Name>]", "create <Name>", "due-dates", "help"];
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
                let mut list_name = remainder.join(" ");
                if list_name.is_empty() {
                    let lists: Vec<BoardList> = self.command_exec.get_all_board_lists(None).await.result.unwrap_or(vec![]);
                    if lists.is_empty() {
                        println!("Found no lists to select");
                        return;
                    } else {
                        let list_names: Vec<&str> = lists.iter().map(|list| list.name.as_str()).collect::<Vec<_>>();
                        let index: usize = self.get_selection_from_prompt(&list_names);
                        list_name = list_names.get(index).unwrap_or(&"").to_string();
                    }
                }
                let list_result = self.command_exec.select_board_list(&list_name, None).await;
                println!("{}", list_result.result_string.unwrap());
                if let CommandResultCode::Success = list_result.result_code {
                    self.current_list.replace(list_result.result.unwrap());
                    self.current_card.take();
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

            "due-dates" => {
                let cards_result = self.command_exec.get_all_list_cards(None).await;
                println!("{}", cards_result.result_string.unwrap());
                if let CommandResultCode::Success = cards_result.result_code {
                    println!("Card Due Dates:");
                    for card in cards_result.result.unwrap() {
                        let due_date: String = if card.due_date_instant_seconds == 0 {
                            "No Due Date".to_string()
                        } else {
                            let datetime = Local.timestamp(card.due_date_instant_seconds, 0);
                            format!("{}", datetime.to_rfc2822())
                        };
                        println!("  - {due}\n      {name}\n", due = due_date, name = card.name);
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
        let available_commands = vec!["create <Name>", "get-all", "select [<Name>]", "get-description", "edit-description <Text>", "move-to-list [<ListName>]", "get-labels", "add-label [<LabelName>]", "remove-label [<LabelName>]", "get-due-date", "set-due-date <yyyy-mm-dd hh:mm:ss>", "set-due-complete", "get-comments", "add-comment <Text>", "help"];
        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "create" => {
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


            "select" => {
                let remainder: Vec<&str> = input_iter.collect();
                let mut card_name = remainder.join(" ");
                if card_name.is_empty() {
                    let cards: Vec<Card> = self.command_exec.get_all_list_cards(None).await.result.unwrap_or(vec![]);
                    if cards.is_empty() {
                        println!("Found no cards to select");
                        return;
                    } else {
                        let card_names: Vec<&str> = cards.iter().map(|card| card.name.as_str()).collect::<Vec<_>>();
                        let index: usize = self.get_selection_from_prompt(&card_names);
                        card_name = card_names.get(index).unwrap_or(&"").to_string();
                    }
                }
                let card_result = self.command_exec.select_list_card(&card_name, None).await;
                println!("{}", card_result.result_string.unwrap());
                if let CommandResultCode::Success = card_result.result_code {
                    self.current_card.replace(card_result.result.unwrap());
                    let description: String = self.current_card.clone().unwrap().description;
                    println!("Card Description: \n{}", description);
                }
            }

            "get-all" => {
                let cards_result = self.command_exec.get_all_list_cards(None).await;
                println!("{}", cards_result.result_string.unwrap());
                if let CommandResultCode::Success = cards_result.result_code {
                    println!("Cards:");
                    for card in cards_result.result.unwrap() {
                        println!("  - {}", card.name);
                    }
                }

            }


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
                    let mut list_name = remainder.join(" ");
                    if list_name.is_empty() {
                        let lists: Vec<BoardList> = self.command_exec.get_all_board_lists(None).await.result.unwrap_or(vec![]);
                        if lists.is_empty() {
                            println!("Found no lists to move to");
                            return;
                        } else {
                            let list_names: Vec<&str> = lists.iter().map(|list| list.name.as_str()).collect::<Vec<_>>();
                            let index: usize = self.get_selection_from_prompt(&list_names);
                            list_name = list_names.get(index).unwrap_or(&"").to_string();
                        }
                    }
                    let card_result = self.command_exec.move_card_to_list(card, &list_name).await;
                    println!("{}", card_result.result_string.unwrap());
                    if let CommandResultCode::Success = card_result.result_code {
                        self.current_card.replace(card_result.result.unwrap());
                        let list_result = self.command_exec.select_board_list(&list_name, None).await;
                        self.current_list.replace(list_result.result.unwrap());
                    }
                }

            }

            "get-labels" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card = self.current_card.clone().unwrap();
                    let labels_result = self.command_exec.get_card_labels(&card).await;
                    println!("{}", labels_result.result_string.unwrap());
                    if let CommandResultCode::Success = labels_result.result_code {
                        let labels: Vec<CardLabel> = labels_result.result.unwrap();
                        for label in labels {
                            println!("  {name}: {color}", name = label.name, color = label.color);
                        }
                    }
                }

            }

            "add-label" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card: Card = self.current_card.clone().unwrap();
                    let remainder: Vec<&str> = input_iter.collect();
                    let mut label_name = remainder.join(" ");
                    if label_name.is_empty() {
                        let labels: Vec<CardLabel> = self.command_exec.get_all_board_labels(None).await.result.unwrap_or(vec![]);
                        if labels.is_empty() {
                            println!("Found no labels to add");
                            return;
                        } else {
                            let label_names: Vec<&str> = labels.iter().map(|label| label.name.as_str()).collect::<Vec<_>>();
                            let index: usize = self.get_selection_from_prompt(&label_names);
                            label_name = label_names.get(index).unwrap_or(&"").to_string();
                        }
                    }
                    let card_result = self.command_exec.add_card_label(card, &label_name).await;
                    println!("{}", card_result.result_string.unwrap());
                    if let CommandResultCode::Success = card_result.result_code {
                        self.current_card.replace(card_result.result.unwrap());
                    }
                }

            }

            "remove-label" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card: Card = self.current_card.clone().unwrap();
                    let remainder: Vec<&str> = input_iter.collect();
                    let mut label_name = remainder.join(" ");
                    if label_name.is_empty() {
                        let labels: Vec<CardLabel> = self.command_exec.get_card_labels(&card).await.result.unwrap_or(vec![]);
                        if labels.is_empty() {
                            println!("Found no labels to remove");
                            return;
                        } else {
                            let label_names: Vec<&str> = labels.iter().map(|label| label.name.as_str()).collect::<Vec<_>>();
                            let index: usize = self.get_selection_from_prompt(&label_names);
                            label_name = label_names.get(index).unwrap_or(&"").to_string();
                        }
                    }
                    let card_result = self.command_exec.remove_card_label(card, &label_name).await;
                    println!("{}", card_result.result_string.unwrap());
                    if let CommandResultCode::Success = card_result.result_code {
                        self.current_card.replace(card_result.result.unwrap());
                    }
                }

            }

            "get-due-date" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card: Card = self.current_card.clone().unwrap();
                    let due_date: String = if card.due_date_instant_seconds == 0 {
                        "No Due Date".to_string()
                    } else {
                        let datetime = Local.timestamp(card.due_date_instant_seconds, 0);
                        format!("{}", datetime.to_rfc2822())
                    };
                    println!("Card Due:\n  {}", due_date);
                }
            }

            "set-due-date" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let mut card: Card = self.current_card.clone().unwrap();
                    let remainder: Vec<&str> = input_iter.collect();
                    let due_string = remainder.join(" ");
                    if due_string.is_empty() {
                        self.print_invalid_command(Some(String::from("You must provide a new due date")));
                        self.print_available_commands(&available_commands);
                    } else {
                        let due_datetime: Option<DateTime<Local>> = Local.datetime_from_str(&due_string, "%Y-%m-%d %H:%M:%S").ok();
                        if due_datetime.is_some() {
                            card.due_date_instant_seconds = due_datetime.unwrap().timestamp();
                            let card_result = self.command_exec.update_card(&card).await;
                            println!("{}", card_result.result_string.unwrap());
                            if let CommandResultCode::Success = card_result.result_code {
                                self.current_card.replace(card_result.result.unwrap());
                            }
                        } else {
                            println!("Unable to parse the due date given");
                        }
                    }
                }
            }

            "set-due-complete" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let mut card: Card = self.current_card.clone().unwrap();
                    card.due_complete = true;
                    let card_result = self.command_exec.update_card(&card).await;
                    println!("{}", card_result.result_string.unwrap());
                    if let CommandResultCode::Success = card_result.result_code {
                        self.current_card.replace(card_result.result.unwrap());
                    }
                }
            }

            "get-comments" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card = self.current_card.clone().unwrap();
                    let comments_result = self.command_exec.get_card_comments(Some(card)).await;
                    println!("{}", comments_result.result_string.unwrap());
                    if let CommandResultCode::Success = comments_result.result_code {
                        let comments: Vec<CardComment> = comments_result.result.unwrap();
                        for comment in comments {
                            let comment_date: String = if comment.comment_time_instant_seconds == 0 {
                                "No Due Date".to_string()
                            } else {
                                let datetime = Local.timestamp(comment.comment_time_instant_seconds, 0);
                                format!("{}", datetime.to_rfc2822())
                            };
                            println!("{name} - {date}\n  {text}\n", date = comment_date, name = comment.commenter_name, text = comment.text);
                        }
                    }
                }
            }

            "add-comment" => {
                if self.current_card.is_none() {
                    println!("No card has been selected");
                    self.print_available_commands(&available_commands);
                } else {
                    let card: Card = self.current_card.clone().unwrap();
                    let remainder: Vec<&str> = input_iter.collect();
                    let text = remainder.join(" ");
                    if text.is_empty() {
                        self.print_invalid_command(Some(String::from("You must provide comment text")));
                        self.print_available_commands(&available_commands);
                    } else {
                        let comment_result = self.command_exec.add_card_comment(Some(card), &text).await;
                        println!("{}", comment_result.result_string.unwrap());
                    }
                }
            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            }

        }

    }

    async fn handle_checklist_command(&mut self, mut input_iter: std::str::SplitAsciiWhitespace<'_>) {
        let available_commands = vec!["get-all", "select [<Name>]", "create <Name>", "get-tasks", "add-task <Name>", "complete-task [<Name>]", "help"];
        match input_iter.next().unwrap_or("") {
            "help" => self.print_available_commands(&available_commands),

            "get-all" => {
                let checklists_results = self.command_exec.get_card_checklists(None).await;
                println!("{}", checklists_results.result_string.unwrap());
                match checklists_results.result_code {
                    CommandResultCode::Success => {
                        let checklists: Vec<CardChecklist> = checklists_results.result.unwrap();
                        println!("Checklists: ");
                        for checklist in checklists {
                            println!("  - {name}", name = checklist.name);
                        }
                    }

                    CommandResultCode::Failed => {
                        println!("Command Failed. Do you have a card selected?");
                    }
                }
            }

            "select" => {
                let remainder: Vec<&str> = input_iter.collect();
                let mut checklist_name = remainder.join(" ");
                if checklist_name.is_empty() {
                    let checklists: Vec<CardChecklist> = self.command_exec.get_card_checklists(None).await.result.unwrap_or(vec![]);
                    if checklists.is_empty() {
                        println!("Found no checklists to select");
                        return;
                    } else {
                        let checklist_names: Vec<&str> = checklists.iter().map(|checklist| checklist.name.as_str()).collect::<Vec<_>>();
                        let index: usize = self.get_selection_from_prompt(&checklist_names);
                        checklist_name = checklist_names.get(index).unwrap_or(&"").to_string();
                    }
                }
                let checklist_result = self.command_exec.select_card_checklist(None, &checklist_name).await;
                println!("{}", checklist_result.result_string.unwrap());
                if let CommandResultCode::Success = checklist_result.result_code {
                    self.current_checklist.replace(checklist_result.result.unwrap());
                }
            }

            "create" => {
                let remainder: Vec<&str> = input_iter.collect();
                let checklist_name = remainder.join(" ");
                if checklist_name.is_empty() {
                    self.print_invalid_command(Some(String::from("You must provide a name")));
                    self.print_available_commands(&available_commands);
                } else {
                    let checklist_result = self.command_exec.create_card_checklists(None, &checklist_name).await;
                    println!("{}", checklist_result.result_string.unwrap());
                }
            }

            "get-tasks" => {
                let tasks_results = self.command_exec.get_checklist_tasks(None).await;
                println!("{}", tasks_results.result_string.unwrap());
                match tasks_results.result_code {
                    CommandResultCode::Success => {
                        let tasks: Vec<CardChecklistTask> = tasks_results.result.unwrap();
                        println!("Tasks: ");
                        for task in tasks {
                            let complete = if task.is_complete {
                                "complete"
                            } else {
                                "incomplete"
                            };
                            println!("  - [{complete}] {name}", complete = complete, name = task.name);
                        }
                    }

                    CommandResultCode::Failed => {
                        println!("Command Failed. Do you have a checklist selected?");
                    }
                }
            }

            "complete-task" => {
                let remainder: Vec<&str> = input_iter.collect();
                let mut task_name = remainder.join(" ");
                if task_name.is_empty() {
                    let tasks: Vec<CardChecklistTask> = self.command_exec.get_checklist_tasks(None).await.result.unwrap_or(vec![]);
                    if tasks.is_empty() {
                        println!("Found no tasks to complete");
                        return;
                    } else {
                        let task_names: Vec<&str> = tasks.iter().map(|task| task.name.as_str()).collect::<Vec<_>>();
                        let index: usize = self.get_selection_from_prompt(&task_names);
                        task_name = task_names.get(index).unwrap_or(&"").to_string();
                    }
                }
                let tasks_result = self.command_exec.get_checklist_tasks(None).await;
                if let CommandResultCode::Success = tasks_result.result_code {
                    let tasks: Vec<CardChecklistTask> = tasks_result.result.unwrap();
                    for mut task in tasks {
                        if task.name.eq_ignore_ascii_case(&task_name) {
                            task.is_complete = true;
                            let task_result = self.command_exec.update_checklist_task(None, task).await;
                            println!("{}", task_result.result_string.unwrap());
                            break;
                        }
                    }
                }
            }

            "add-task" => {
                let remainder: Vec<&str> = input_iter.collect();
                let task_name = remainder.join(" ");
                if task_name.is_empty() {
                    self.print_invalid_command(Some(String::from("You must provide a name")));
                    self.print_available_commands(&available_commands);
                } else {
                    let task_result = self.command_exec.create_checklist_task(None, &task_name).await;
                    println!("{}", task_result.result_string.unwrap());
                }
            }

            _ => {
                self.print_invalid_command(None);
                self.print_available_commands(&available_commands);
            }
        }
    }
}

pub async fn run(config: Option<serde_json::Value>) {
    let command_exec = CommandExecutor::new(config).await;
    let mut cli = InteractiveCli {
        command_exec: command_exec,
        current_board: None,
        current_list: None,
        current_card: None,
        current_checklist: None,
    };

    let available_commands = vec!["board", "label", "list", "card", "checklist", "exit", "help"];

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

            "checklist" => {
                cli.handle_checklist_command(input_iter).await;
            }

            _ => {
                cli.print_invalid_command(None);
                cli.print_available_commands(&available_commands);
            }
        }
    }
}
