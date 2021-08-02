#include "command_executor.h"

#include <iostream>

CommandExecutor::CommandExecutor()
{
}

CommandExecutor::~CommandExecutor()
{
}

std::shared_ptr<CommandResult> CommandExecutor::ParseExecute(const std::string &command)
{
    std::cout << "Parsing and Executing Command" << std::endl;
    return NULL;
}