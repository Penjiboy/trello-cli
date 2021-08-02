#pragma once

#include <string>
#include <vector>
#include <ctime>

struct CardLabel
{
    std::string name;
    std::string id;
    std::string color;
};

struct CardComment
{
    std::string text;
    std::string commenterName;
    std::string commentId;
    std::tm commentTime;
};

struct CardChecklistTask
{
    std::string id;
    std::string name;
    bool isComplete;
};

struct CardChecklist
{
    std::string id;
    std::string name;
    std::vector<CardChecklistTask> tasks;
};

struct Card
{
    std::string id;
    std::string name;
    std::string description;
    std::tm dueDate;
    std::vector<CardLabel> labels;
    std::vector<CardComment> comments;
    std::vector<CardChecklist> checklists;
};

struct BoardList
{
    std::string id;
    std::string name;
    std::vector<Card> cards;
};

struct Board
{
    std::string id;
    std::string name;
    std::string workspace;
    std::vector<CardLabel> labels;
    std::vector<BoardList> lists;
};