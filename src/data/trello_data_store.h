#pragma once

#include "data_repository.h"

#include <string>

class TrelloDataStore : public DataRepository
{
public:
    TrelloDataStore(/* args */);
    ~TrelloDataStore();

private:
    /* data */
    const std::string kUrlBase = "https://api.trello.com/1";
};