#pragma once

#include "../data/board.h"
#include <memory>
#include <vector>
#include <string>

class BoardService
{
public:
    ~BoardService();
    static std::shared_ptr<BoardService> GetInstance();

    std::shared_ptr<std::vector<const Board>> GetAllBoards();
    std::shared_ptr<const Board> CreateNewBoard(const std::string &name);
    std::shared_ptr<std::vector<const CardLabel>> GetBoardLabels();
    std::shared_ptr<const CardLabel> CreateBoardLabel(const std::string &name);
    std::shared_ptr<std::vector<const BoardList>> GetAllBoardLists();
    std::shared_ptr<const BoardList> CreateNewBoardList(const std::string &name);

    int GetListCardCount(const BoardList &list);
    std::shared_ptr<std::vector<Card>> GetAllCardsInList(const BoardList &list);
    std::shared_ptr<std::vector<tm>> GetListDueDates(const BoardList &list);
    std::shared_ptr<Card> CreateNewCard(const BoardList &list, const std::string &name);
    std::shared_ptr<Card> MoveCardToList(const BoardList &originalList, const BoardList &destinationList, const Card &card);

    void SetCurrentBoard(const Board &board);

private:
    BoardService(/* args */);

    static std::shared_ptr<BoardService> s_Instance;
    std::shared_ptr<Board> m_CurrentBoard = NULL;
};
