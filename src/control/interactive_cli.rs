use crate::control::command_executor as CommandExecutor;
use crate::control::*;
use crate::data::*;

use std::io;

fn print_prompt(
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
    print!("{}", path);
}

pub fn run() {
    let mut current_board: Option<Board> = None;
    let mut current_list: Option<BoardList> = None;
    let mut current_card: Option<Card> = None;
    let mut current_checklist: Option<CardChecklist> = None;

    loop {
        print_prompt(&current_board, &current_list, &current_card, &current_checklist);
        let mut input = String::new();
        let read_result = io::stdin().read_line(&mut input);
        if read_result.is_err() {
            println!("Error while reading input: {}", read_result.unwrap_err());
            break;
        }
    }
}
