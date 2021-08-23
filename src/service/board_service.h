#pragma once

#include "../data/board.h"
#include "../data/data_repository.h"

#include <memory>
#include <vector>
#include <string>

class BoardService
{
public:
    static std::shared_ptr<BoardService> GetInstance();

    std::shared_ptr<const std::vector<Board>> GetAllBoards();
    std::shared_ptr<const Board> CreateNewBoard(const std::string &name);
    std::shared_ptr<const std::vector<CardLabel>> GetBoardLabels();
    std::shared_ptr<const CardLabel> CreateBoardLabel(const std::string &name);
    std::shared_ptr<const std::vector<BoardList>> GetAllBoardLists();
    std::shared_ptr<const BoardList> CreateNewBoardList(const std::string &name);

    int GetListCardCount(const BoardList &list);
    std::shared_ptr<const std::vector<Card>> GetAllCardsInList(const BoardList &list);
    std::shared_ptr<const std::vector<tm>> GetListDueDates(const BoardList &list);
    std::shared_ptr<const Card> CreateNewCard(const BoardList &list, const std::string &name);
    std::shared_ptr<const Card> MoveCardToList(const BoardList &originalList, const BoardList &destinationList, const Card &card);

    void SetCurrentBoard(const Board &board);

private:
    BoardService() { this->m_DataRepository = DataRepository::GetInstance(); };
    std::shared_ptr<DataRepository> m_DataRepository;
    std::shared_ptr<Board> m_CurrentBoard = NULL;
};
