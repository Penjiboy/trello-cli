#pragma once

#include "../data/board.h"

#include <string>
#include <vector>
#include <memory>

class CardService
{
public:
    static std::shared_ptr<CardService> GetInstance();
    ~CardService();

    void SetCurrentCard(const Card &card);

    std::shared_ptr<std::string> GetCardDescription();
    std::shared_ptr<Card> EditCardDescription(const std::string &description);
    std::shared_ptr<std::vector<CardChecklist>> GetCardChecklists();
    std::shared_ptr<Card> CreateCardChecklist(const std::string &checklistName);
    std::shared_ptr<std::vector<CardComment>> GetCardComments();
    std::shared_ptr<Card> AddCardComment(CardComment &comment);
    std::shared_ptr<std::vector<CardLabel>> GetCardLabels();
    std::shared_ptr<Card> AddCardLabel(const CardLabel &label);
    std::shared_ptr<Card> RemoveCardLabel(const CardLabel &label);
    std::shared_ptr<std::tm> GetCardDueDate();
    std::shared_ptr<Card> SetCardDueDate(const std::tm &dueDate);

    // Checklist Methods
    std::shared_ptr<std::vector<CardChecklistTask>> GetChecklistTasks(const CardChecklist &checklist);
    std::shared_ptr<CardChecklist> AddChecklistTask(const CardChecklist &checklist, CardChecklistTask &task);
    std::shared_ptr<CardChecklist> CompleteChecklistTask(const CardChecklist &checklist, CardChecklistTask &task);

private:
    CardService(/* args */);

    static std::shared_ptr<CardService> s_Instance = NULL;
    std::shared_ptr<Card> m_CurrentCard = NULL;
    std::shared_ptr<Board> m_CurrentBoard = NULL;
    /* data */
};

CardService::CardService(/* args */)
{
}

CardService::~CardService()
{
}
