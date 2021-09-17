#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "trello_data_store.h"

#include "httplib.h"

#include <iostream>
#include <fstream>

TrelloDataStore::TrelloDataStore()
{
    // Read and set the key and token
    std::fstream keyFile;
    keyFile.open(kPathToKey, std::ios::in); // open a file to read
    if (keyFile.is_open())
    {
        std::string tp;
        while (std::getline(keyFile, tp)) 
        {
            m_key = tp;
        }
    }
    keyFile.close();

    std::fstream tokenFile;
    tokenFile.open(kPathToToken, std::ios::in);
    if (tokenFile.is_open())
    {
        std::string tp;
        while (std::getline(tokenFile, tp))
        {
            m_token = tp;
        }
    }
    tokenFile.close();
}

std::shared_ptr<TrelloDataStore> TrelloDataStore::GetInstance()
{
    static std::shared_ptr<TrelloDataStore> instance(new TrelloDataStore());
    return instance;
}

std::shared_ptr<const std::vector<Board>> TrelloDataStore::GetAllBoards()
{
    // Send HTTP Request
    httplib::Client client("https://www.google.com");
    // auto response = client.Get("/");
    // std::cout << "Response Status: " << response->status << std::endl;
    // std::cout << "Response Body: " << response->body << std::endl;

    // Parse Response

    // Return list of boards
}