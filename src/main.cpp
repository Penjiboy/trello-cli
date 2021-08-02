#include <iostream>
#include "control/command_executor.h"

int main(int argc, char *argv[])
{
    std::cout << "Hello Trello CLI!" << std::endl;

    CommandExecutor *executor = new CommandExecutor();
    std::shared_ptr<CommandResult> result = executor->ParseExecute("Hello executor");
    delete executor;
}