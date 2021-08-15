#pragma once

#include <string>
#include <vector>
#include <ctime>

struct BoardType
{
    std::string id;
};
struct CardLabel : public BoardType
{
    std::string name;
    std::string color;
};

struct CardComment : public BoardType
{
    std::string text;
    std::string commenterName;
    std::tm commentTime;
};

struct CardChecklistTask : public BoardType
{
    std::string name;
    bool isComplete;
};

struct CardChecklist : public BoardType
{
    std::string name;
    std::vector<CardChecklistTask> tasks;
};

struct Card : public BoardType
{
    std::string name;
    std::string description;
    std::tm dueDate;
    std::vector<CardLabel> labels;
    std::vector<CardComment> comments;
    std::vector<CardChecklist> checklists;
};

struct BoardList : public BoardType
{
    std::string name;
    std::vector<Card> cards;
};

struct Board : public BoardType
{
    std::string name;
    std::string workspace;
    std::vector<CardLabel> labels;
    std::vector<BoardList> lists;
};