#pragma once

#include "../data/board.h"

#include <string>
#include <vector>
#include <memory>

class CardService
{
public:
    static std::shared_ptr<CardService> GetInstance();

    void SetCurrentCard(const Card &card);

    std::shared_ptr<const std::string> GetCardDescription();
    std::shared_ptr<const Card> EditCardDescription(const std::string &description);
    std::shared_ptr<const std::vector<CardChecklist>> GetCardChecklists();
    std::shared_ptr<const Card> CreateCardChecklist(const std::string &checklistName);
    std::shared_ptr<const std::vector<CardComment>> GetCardComments();
    std::shared_ptr<const Card> AddCardComment(CardComment &comment);
    std::shared_ptr<const std::vector<CardLabel>> GetCardLabels();
    std::shared_ptr<const Card> AddCardLabel(const CardLabel &label);
    std::shared_ptr<const Card> RemoveCardLabel(const CardLabel &label);
    std::shared_ptr<const std::tm> GetCardDueDate();
    std::shared_ptr<const Card> SetCardDueDate(const std::tm &dueDate);

    // Checklist Methods
    std::shared_ptr<const std::vector<CardChecklistTask>> GetChecklistTasks(const CardChecklist &checklist);
    std::shared_ptr<const CardChecklist> AddChecklistTask(const CardChecklist &checklist, CardChecklistTask &task);
    std::shared_ptr<const CardChecklist> CompleteChecklistTask(const CardChecklist &checklist, CardChecklistTask &task);

private:
    CardService() {}

    std::shared_ptr<Card> m_CurrentCard = NULL;
    std::shared_ptr<Board> m_CurrentBoard = NULL;
    /* data */
};
