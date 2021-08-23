#include "data_repository.h"

std::shared_ptr<DataRepository> DataRepository::GetInstance()
{
    static std::shared_ptr<DataRepository> instance(new DataRepository());
    return instance;
}

std::shared_ptr<const std::vector<Board>> DataRepository::GetAllBoards()
{
    return this->m_TrelloDataStore->GetAllBoards();
}
