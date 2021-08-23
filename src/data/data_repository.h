#pragma once

#include "board.h"
#include <memory>
#include <vector>

class DataRepository
{
public:
    DataRepository(/* args */);
    ~DataRepository();

    // Board Methods
    std::shared_ptr<const std::vector<Board>> GetAllBoards();
    std::shared_ptr<const Board> CreateNewBoard(const std::string &name);
    std::shared_ptr<const std::vector<CardLabel>> GetBoardLabels(const Board &board);
    std::shared_ptr<const CardLabel> CreateBoardLabel(const Board &board, const std::string &name);
    std::shared_ptr<const std::vector<BoardList>> GetAllBoardLists(const Board &board);
    std::shared_ptr<const BoardList> CreateNewBoardList(const Board &board, const std::string &name);

    // BoardList Methods
    int GetListCardCount(const Board &board, const BoardList &list);
    std::shared_ptr<const std::vector<Card>> GetAllCards(const Board &board, const BoardList &list);
    std::shared_ptr<const std::vector<tm>> GetListDueDates(const Board &board, const BoardList &list);
    std::shared_ptr<const Card> CreateNewCard(const Board &board, const BoardList &list, const std::string &name);
    std::shared_ptr<const Card> MoveCardToList(const Board &board, const BoardList &originalList, const BoardList &destinationList, const Card &card);

    // Card Methods
    std::shared_ptr<const std::string> GetCardDescription(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<const Card> EditCardDescription(const Board &board, const BoardList &list, const Card &card, const std::string &description);
    std::shared_ptr<const std::vector<CardChecklist>> GetCardChecklists(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<const Card> CreateCardChecklist(const Board &board, const BoardList &list, const Card &card, const std::string &checklistName);
    std::shared_ptr<const std::vector<CardComment>> GetCardComments(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<const Card> AddCardComment(const Board &board, const BoardList &list, const Card &card, CardComment &comment);
    std::shared_ptr<const std::vector<CardLabel>> GetCardLabels(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<const Card> AddCardLabel(const Board &board, const BoardList &list, const Card &card, const CardLabel &label);
    std::shared_ptr<const Card> RemoveCardLabel(const Board &board, const BoardList &list, const Card &card, const CardLabel &label);
    std::shared_ptr<const CardDueDate> GetCardDueDate(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<const Card> SetCardDueDate(const Board &board, const BoardList &list, const Card &card, const std::tm &dueDate);

    // Checklist Methods
    std::shared_ptr<const std::vector<CardChecklistTask>> GetChecklistTasks(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist);
    std::shared_ptr<const CardChecklist> AddChecklistTask(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist, CardChecklistTask &task);
    std::shared_ptr<const CardChecklist> CompleteChecklistTask(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist, CardChecklistTask &task);

private:
    /* data */
};