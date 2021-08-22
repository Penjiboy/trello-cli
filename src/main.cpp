#include <iostream>
#include <memory>
#include "control/command_executor.h"

int main(int argc, char *argv[])
{
    std::cout << "Hello Trello CLI!" << std::endl;
    std::shared_ptr<CommandExecutor> executor = std::make_shared<CommandExecutor>();

    auto boardsResult = executor->GetAllBoards();
    if (boardsResult->resultCode != CommandResultCode::Success)
    {
        std::cout << "Failed to get all boards" << std::endl;
        return 0;
    }

    std::shared_ptr<std::vector<Board>> boards = boardsResult->result;
    for (Board board : *boards)
    {
        std::cout << "Board ID: " << board.trelloId << std::endl;
        std::cout << "Board Name: " << board.name << std::endl << std::endl;
    }
}