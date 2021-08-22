#include "card_service.h"

std::shared_ptr<CardService> CardService::GetInstance()
{
    static std::shared_ptr<CardService> instance(new CardService);
    return instance;
}