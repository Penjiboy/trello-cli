#include "board_service.h"

std::shared_ptr<BoardService> BoardService::GetInstance()
{
    static std::shared_ptr<BoardService> instance(new BoardService());
    return instance;
}