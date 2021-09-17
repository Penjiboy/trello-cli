#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "trello_data_store.h"

#include "httplib.h"

std::shared_ptr<TrelloDataStore> TrelloDataStore::GetInstance()
{
    static std::shared_ptr<TrelloDataStore> instance(new TrelloDataStore());
    return instance;
}

std::shared_ptr<const std::vector<Board>> TrelloDataStore::GetAllBoards()
{
    // Send HTTP Request
    httplib::Client client("https://www.google.com");
    auto response = client.Get("/");
    std::cout << "Response Status: " << response->status << std::endl;
    std::cout << "Response Body: " << response->body << std::endl;

    // Parse Response

    // Return list of boards
}