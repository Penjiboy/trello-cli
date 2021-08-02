#pragma once

#include <string>
#include <vector>
#include <memory>

enum CommandResultCode
{
    Success = 0,
    Failed = 1,
    UnknownCommand = 2,
    NotAuthorized = 3
};

struct CommandResult
{
    CommandResultCode resultCode;
    std::string result;
};

class CommandExecutor
{
public:
    CommandExecutor();
    ~CommandExecutor();
    std::shared_ptr<CommandResult> ParseExecute(const std::string &command);

private:
    /* data */
};
