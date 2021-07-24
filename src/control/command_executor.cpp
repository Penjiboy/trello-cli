#include "command_executor.h"

#include <iostream>

CommandExecutor::CommandExecutor()
{
}

CommandExecutor::~CommandExecutor()
{
}

CommandResult *CommandExecutor::ParseExecute(const std::string &command)
{
    std::cout << "Parsing and Executing Command" << std::endl;
    return NULL;
}