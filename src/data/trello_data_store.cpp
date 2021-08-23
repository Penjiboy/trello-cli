#include "trello_data_store.h"

std::shared_ptr<TrelloDataStore> TrelloDataStore::GetInstance()
{
    static std::shared_ptr<TrelloDataStore> instance(new TrelloDataStore());
    return instance;
}

std::shared_ptr<const std::vector<Board>> TrelloDataStore::GetAllBoards()
{
    // Send HTTP Request

    // Parse Response

    // Return list of boards
}