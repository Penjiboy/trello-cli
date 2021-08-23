#pragma once

#include "../service/board_service.h"
#include "../service/card_service.h"
#include "../data/board.h"

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

template <typename T>
struct CommandResult
{
    CommandResultCode resultCode;
    std::shared_ptr<const std::vector<T>> result;
    std::string resultString;
};

class CommandExecutor
{
public:
    CommandExecutor();
    ~CommandExecutor();

    // Board Methods
    std::shared_ptr<CommandResult<Board>> GetAllBoards();
    std::shared_ptr<CommandResult<Board>> CreateNewBoard(const std::string &name);
    std::shared_ptr<CommandResult<CardLabel>> GetBoardLabels(const Board &board);
    std::shared_ptr<CommandResult<CardLabel>> CreateBoardLabel(const Board &board, const std::string &name);
    std::shared_ptr<CommandResult<BoardList>> GetAllBoardLists(const Board &board);
    std::shared_ptr<CommandResult<BoardList>> CreateNewBoardList(const Board &board, const std::string &name);

    // BoardList Methods
    std::shared_ptr<CommandResult<int>> GetListCardCount(const Board &board, const BoardList &list);
    std::shared_ptr<CommandResult<Card>> GetAllCards(const Board &board, const BoardList &list);
    std::shared_ptr<CommandResult<CardDueDate>> GetListDueDates(const Board &board, const BoardList &list);
    std::shared_ptr<CommandResult<Card>> CreateNewCard(const Board &board, const BoardList &list, const std::string &name);
    std::shared_ptr<CommandResult<Card>> MoveCardToList(const Board &board, const BoardList &originalList, const BoardList &destinationList, const Card &card);

    // Card Methods
    std::shared_ptr<CommandResult<std::string>> GetCardDescription(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult<Card>> EditCardDescription(const Board &board, const BoardList &list, const Card &card, const std::string &description);
    std::shared_ptr<CommandResult<CardChecklist>> GetCardChecklists(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult<CardChecklist>> CreateCardChecklist(const Board &board, const BoardList &list, const Card &card, const std::string &checklistName);
    std::shared_ptr<CommandResult<CardComment>> GetCardComments(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult<CardComment>> AddCardComment(const Board &board, const BoardList &list, const Card &card, CardComment &comment);
    std::shared_ptr<CommandResult<CardLabel>> GetCardLabels(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult<CardLabel>> AddCardLabel(const Board &board, const BoardList &list, const Card &card, const CardLabel &label);
    std::shared_ptr<CommandResult<Card>> RemoveCardLabel(const Board &board, const BoardList &list, const Card &card, const CardLabel &label);
    std::shared_ptr<CommandResult<CardDueDate>> GetCardDueDate(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult<CardDueDate>> SetCardDueDate(const Board &board, const BoardList &list, const Card &card, const std::tm &dueDate);

    // Checklist Methods
    std::shared_ptr<CommandResult<CardChecklistTask>> GetChecklistTasks(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist);
    std::shared_ptr<CommandResult<CardChecklistTask>> AddChecklistTask(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist, CardChecklistTask &task);
    std::shared_ptr<CommandResult<CardChecklistTask>> CompleteChecklistTask(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist, CardChecklistTask &task);

private:
    /* data */
    std::shared_ptr<BoardService> m_BoardService;
    std::shared_ptr<CardService> m_CardService;
};
