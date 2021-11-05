use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc, options::ClientOptions, options::InsertManyOptions, options::UpdateOptions, Client,
    Collection, Database, bson::oid, bson::Document
};
use serde_json::{Map, Value};

use std::sync::Once;
use std::collections::HashMap;

static START: Once = Once::new();

static mut db: Option<Database> = None;

pub struct MongoDataStore {}

impl MongoDataStore {
    pub async fn init(config: Option<Value>) {
        // Parse a connection string into an options struct
        let mut connection_string = String::from("mongodb://root:rootpassword@localhost:32392");
        if config.is_some() {
            let config_object = config.as_ref().unwrap().as_object().unwrap();
            let host = config_object.get("mongodb").unwrap().as_object().unwrap().get("host").unwrap().as_str().unwrap();
            let port = config_object.get("mongodb").unwrap().as_object().unwrap().get("port").unwrap().as_i64().unwrap();
            let password = config_object.get("mongodb").unwrap().as_object().unwrap().get("password").unwrap().as_str().unwrap();
            let username = config_object.get("mongodb").unwrap().as_object().unwrap().get("username").unwrap().as_str().unwrap();
            connection_string = format!("mongodb://{username}:{password}@{host}:{port}", username = username, password = password, host = host, port = port);
        }
        let mut client_options =
            ClientOptions::parse(connection_string)
                .await
                .unwrap();

        // Manually set an option
        client_options.app_name = Some(String::from("TrelloData"));

        START.call_once(|| {
            // Get a handle to the deployment
            let client = Client::with_options(client_options).unwrap();
            let _db = client.database("trelloData");
            unsafe {
                db = Some(_db);
            }
        });
    }

    pub async fn sync_boards(trello_boards: Vec<Board>) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let boards_collection: Collection<Board>;

        unsafe {
            boards_collection = db.clone().unwrap().collection::<Board>("boards");
        }

        let mut boards_with_ids: Vec<Board> = vec![];
        let existing_boards: Vec<Board> = MongoDataStore::get_all_boards().await?;
        
        // Maybe create a map of existing_board_by_trello_id and then use the map to help synchronize in the loop?
        let mut existing_board_by_trello_id: HashMap<String, Board> = HashMap::new();

        for board in existing_boards {
            existing_board_by_trello_id.insert(board.clone()._id.trello_id.unwrap(), board);
        }

        for mut board in trello_boards {
            let existing_board = existing_board_by_trello_id.get(&board._id.trello_id.clone().unwrap());
            if existing_board.is_some() {
                board._id.local_id.replace(existing_board.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = board.to_doc::<Board>(true);
                let update_result = boards_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": board._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                board._id.local_id.replace(object_id.to_hex());
                let insert_result = boards_collection.insert_one(board.clone(), None).await?;
            }

            boards_with_ids.push(board);
        }

        Ok(boards_with_ids)
    }
}

#[async_trait]
impl DataStore for MongoDataStore {
    async fn get_all_boards() -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let boards: Vec<Board>;

        unsafe {
            let boards_collection: Collection<Board> =
                db.clone().unwrap().collection::<Board>("boards");
            let cursor = boards_collection.find(None, None).await?;
            boards = cursor.try_collect().await?;
        }

        Ok(boards)
    }

    async fn create_board(name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_all_board_labels(board_id: &str) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn delete_board_label(label_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn update_board_label(label_id: &str, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_board_label(board_id: &str, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_all_board_lists(board_id: &str) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_board_list(board_id: &str, name: &str) -> Result<BoardList, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_all_list_cards(list_id: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_list_card(list_id: &str, name: &str) -> Result<Card, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn update_card(card: &Card) -> Result<Card, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_card_comments(card_id: &str) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn add_card_comment(card_id: &str, text: &str) -> Result<CardComment, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_card_checklists(card_id: &str) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_card_checklist(card_id: &str, name: &str) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_checklist_tasks(checklist_id: &str) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_checklist_task(checklist_id: &str, name: &str) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn update_checklist_task(card_id: &str, task: &CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }
}

async fn test_mongo_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://root:rootpassword@localhost:32392").await?;

    // Manually set an option
    client_options.app_name = Some(String::from("TrelloData"));

    // Get a handle to the deployment
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment
    println!("Database names: ");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database
    let _db = client.database("trelloData");

    // List the names of the collections in that database
    println!("Collections in trelloData: ");
    for collection_name in _db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}

trait ToDocument {
    /**
     * The `is_update_op` flag will determine how we construct the document. 
     * If true, then all the fields (except for _id) will be wrapped in a $set object, 
     * returning a document that can be passed directly into the update command parameter.
     * 
     * If false, the returned document will be as if it came from a find operation
     */
    fn to_doc<T>(&self, is_update_op: bool) -> Document;
    fn to_docs<T: ToDocument>(objects: Vec<T>, is_update_op: bool) -> Vec<Document> {
        let mut result: Vec<Document> = vec![];
        for object in objects {
            let doc = object.to_doc::<T>(is_update_op);
            result.push(doc);
        }

        result
    }
}

impl ToDocument for Board {
    fn to_doc<Board>(&self, is_update_op: bool) -> Document {
        return if is_update_op {
            doc! {
                "$set": doc! {
                    "name": self.name.clone()
                }
            }
        } else {
            doc! {
                "_id": doc! {
                    "trello_id": self._id.trello_id.clone().unwrap(),
                    "local_id": self._id.local_id.clone().unwrap()
                },
                "name": self.name.clone()
            }
        }
    }
}