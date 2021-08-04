#pragma once

#include "../data/board.h"
#include <memory>
#include <vector>
#include <string>

class BoardService
{
public:
    BoardService(/* args */);
    ~BoardService();

    std::shared_ptr<std::vector<const Board>> GetAllBoards();
    std::shared_ptr<const Board> CreateNewBoard(const std::string &name);
    std::shared_ptr<std::vector<const CardLabel>> GetBoardLabels(const Board &board);
    std::shared_ptr<std::vector<const CardLabel>> GetBoardLabels();
    std::shared_ptr<const CardLabel> CreateBoardLabel(const Board &board, const std::string &name);
    std::shared_ptr<const CardLabel> CreateBoardLabel(const std::string &name);
    std::shared_ptr<std::vector<const BoardList>> GetAllBoardLists(const Board &board);
    std::shared_ptr<std::vector<const BoardList>> GetAllBoardLists();
    std::shared_ptr<const BoardList> CreateNewBoardList(const Board &board, const std::string &name);
    std::shared_ptr<const BoardList> CreateNewBoardList(const std::string &name);

    void SetCurrentBoard(const Board &board);

private:
    std::shared_ptr<Board> CurrentBoard;
};
