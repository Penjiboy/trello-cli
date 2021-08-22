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

}