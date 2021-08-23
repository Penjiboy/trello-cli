#include "board_service.h"

#include <iostream>

std::shared_ptr<BoardService> BoardService::GetInstance()
{
    static std::shared_ptr<BoardService> instance(new BoardService());
    return instance;
}

std::shared_ptr<const std::vector<Board>> BoardService::GetAllBoards()
{
    std::cout << "Getting boards" << std::endl;
}