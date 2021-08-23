#include "command_executor.h"

#include <iostream>

CommandExecutor::CommandExecutor()
{
    this->m_BoardService = BoardService::GetInstance();
    this->m_CardService = CardService::GetInstance();
}

CommandExecutor::~CommandExecutor()
{
}

std::shared_ptr<CommandResult<Board>> CommandExecutor::GetAllBoards()
{
    std::shared_ptr<const std::vector<Board>> boards = this->m_BoardService->GetAllBoards();
    std::shared_ptr<CommandResult<Board>> result(new CommandResult<Board>);
    if (!boards)
    {
        result->resultCode = CommandResultCode::Failed;
        return result;
    }
    
    result->resultCode = CommandResultCode::Success;
    result->result = boards;
    
}