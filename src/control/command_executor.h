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

struct CommandResult
{
    CommandResultCode resultCode;
    std::shared_ptr<std::vector<BoardType>> result;
    std::string resultString;
};

class CommandExecutor
{
public:
    CommandExecutor();
    ~CommandExecutor();
    std::shared_ptr<CommandResult> ParseExecute(const std::string &command);

    // Board Methods
    std::shared_ptr<CommandResult> GetAllBoards();
    std::shared_ptr<CommandResult> CreateNewBoard(const std::string &name);
    std::shared_ptr<CommandResult> GetBoardLabels(const Board &board);
    std::shared_ptr<CommandResult> CreateBoardLabel(const Board &board, const std::string &name);
    std::shared_ptr<CommandResult> GetAllBoardLists(const Board &board);
    std::shared_ptr<CommandResult> CreateNewBoardList(const Board &board, const std::string &name);

    // BoardList Methods
    std::shared_ptr<CommandResult> GetListCardCount(const Board &board, const BoardList &list);
    std::shared_ptr<CommandResult> GetAllCards(const Board &board, const BoardList &list);
    std::shared_ptr<CommandResult> GetListDueDates(const Board &board, const BoardList &list);
    std::shared_ptr<CommandResult> CreateNewCard(const Board &board, const BoardList &list, const std::string &name);
    std::shared_ptr<CommandResult> MoveCardToList(const Board &board, const BoardList &originalList, const BoardList &destinationList, const Card &card);

    // Card Methods
    std::shared_ptr<CommandResult> GetCardDescription(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult> EditCardDescription(const Board &board, const BoardList &list, const Card &card, const std::string &description);
    std::shared_ptr<CommandResult> GetCardChecklists(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult> CreateCardChecklist(const Board &board, const BoardList &list, const Card &card, const std::string &checklistName);
    std::shared_ptr<CommandResult> GetCardComments(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult> AddCardComment(const Board &board, const BoardList &list, const Card &card, CardComment &comment);
    std::shared_ptr<CommandResult> GetCardLabels(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult> AddCardLabel(const Board &board, const BoardList &list, const Card &card, const CardLabel &label);
    std::shared_ptr<CommandResult> RemoveCardLabel(const Board &board, const BoardList &list, const Card &card, const CardLabel &label);
    std::shared_ptr<CommandResult> GetCardDueDate(const Board &board, const BoardList &list, const Card &card);
    std::shared_ptr<CommandResult> SetCardDueDate(const Board &board, const BoardList &list, const Card &card, const std::tm &dueDate);

    // Checklist Methods
    std::shared_ptr<CommandResult> GetChecklistTasks(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist);
    std::shared_ptr<CommandResult> AddChecklistTask(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist, CardChecklistTask &task);
    std::shared_ptr<CommandResult> CompleteChecklistTask(const Board &board, const BoardList &list, const Card &card, const CardChecklist &checklist, CardChecklistTask &task);

private:
    /* data */
    std::shared_ptr<BoardService> m_BoardService;
    std::shared_ptr<CardService> m_CardService;
};
