#pragma once

#include "generic_data_store.h"

#include <string>
#include <memory>

class TrelloDataStore : public GenericDataStore
{
public:
    static std::shared_ptr<TrelloDataStore> GetInstance();
    std::shared_ptr<const std::vector<Board>> GetAllBoards();

private:
    /* data */
    TrelloDataStore(/* args */);
    const std::string kUrlBase = "https://api.trello.com/1";
    const std::string kPathToKey = ".config/developer_api_key.txt";
    const std::string kPathToToken = ".config/developer_api_token.txt";

    std::string m_key;
    std::string m_token;
};