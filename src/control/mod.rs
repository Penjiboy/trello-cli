use crate::service::board as BoardService;

pub enum CommandResultCode {
    Success,
    Failed,
}

pub struct CommandResult<T> {
    pub result_code: CommandResultCode,
    pub result: Option<T>,
    pub result_string: Option<String>
}

pub mod command_executor {
    use crate::service::board as BoardService;
    use crate::control::*;
    use crate::data::*;
    
    pub fn init() {
        BoardService::init()
    }

    pub async fn get_all_boards() -> Result<CommandResult<Vec<Board>>, Box<dyn std::error::Error>> {
        let boards_result = BoardService::get_all_boards().await;
        let command_result: CommandResult<Vec<Board>> = match boards_result {
            Ok(boards) => {
                let result_code = CommandResultCode::Success;
                let result_string = format!("Retrieved {} boards", boards.len());
                CommandResult {
                    result_code: result_code,
                    result: Some(boards),
                    result_string: Some(result_string)
                }
            }

            Err(why) => {
                let result_code = CommandResultCode::Failed;
                let result_string = String::from(why.to_string());
                CommandResult {
                    result_code: result_code,
                    result: None,
                    result_string: Some(result_string)
                }
            }
        };

        Ok(command_result)
    }
}