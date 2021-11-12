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
        let boards_collection: Collection<Board>;
        unsafe {
            boards_collection = db.clone().unwrap().collection::<Board>("boards");
        }
        let object_id = oid::ObjectId::new();
        let board: Board = Board {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string()
        };

        let _insert_result = boards_collection.insert_one(board.clone(), None).await?;
        Ok(board)
    }

    async fn get_all_board_labels(board_id: ID) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        let labels: Vec<CardLabel>;

        unsafe {
            let labels_collection: Collection<CardLabel> = db.clone().unwrap().collection::<CardLabel>("labels");
            let cursor = labels_collection.find(doc! {
                "board_id": doc! {
                    "trello_id": board_id.trello_id.unwrap(),
                    "local_id": board_id.local_id.unwrap()
                }
            }, None).await?;
            labels = cursor.try_collect().await?;
        }

        Ok(labels)
    }

    async fn delete_board_label(label_id: ID) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn update_board_label(label_id: ID, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn create_board_label(board_id: ID, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let labels_collection: Collection<CardLabel>;
        unsafe {
            labels_collection = db.clone().unwrap().collection::<CardLabel>("labels");
        }
        let object_id = oid::ObjectId::new();
        let label: CardLabel = CardLabel {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string(),
            color: color.to_string(),
            board_id: board_id
        };

        let _insert_result = labels_collection.insert_one(label.clone(), None).await?;
        Ok(label)
    }

    async fn get_all_board_lists(board_id: ID) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        let lists: Vec<BoardList>;

        unsafe {
            let lists_collection: Collection<BoardList> = db.clone().unwrap().collection::<BoardList>("lists");
            let cursor = lists_collection.find(doc! {
                "board_id": doc! {
                    "trello_id": board_id.trello_id.unwrap(),
                    "local_id": board_id.local_id.unwrap()
                }
            }, None).await?;
            lists = cursor.try_collect().await?;
        }

        Ok(lists)
    }

    async fn create_board_list(board_id: ID, name: &str) -> Result<BoardList, Box<dyn std::error::Error>> {
        let lists_collection: Collection<BoardList>;
        unsafe {
            lists_collection = db.clone().unwrap().collection::<BoardList>("lists");
        }
        let object_id = oid::ObjectId::new();
        let list: BoardList = BoardList {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string(),
            board_id: board_id
        };

        let _insert_result = lists_collection.insert_one(list.clone(), None).await?;
        Ok(list)
    }

    async fn get_all_list_cards(list_id: ID) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let cards: Vec<Card>;

        unsafe {
            let cards_collection: Collection<Card> = db.clone().unwrap().collection::<Card>("cards");
            let cursor = cards_collection.find(doc! {
                "list_id": doc! {
                    "trello_id": list_id.trello_id.unwrap(),
                    "local_id": list_id.local_id.unwrap()
                }
            }, None).await?;
            cards = cursor.try_collect().await?;
        }

        Ok(cards)
    }

    async fn create_list_card(list_id: ID, name: &str) -> Result<Card, Box<dyn std::error::Error>> {
        let cards_collection: Collection<Card>;
        unsafe {
            cards_collection = db.clone().unwrap().collection::<Card>("cards");
        }
        let object_id = oid::ObjectId::new();
        let card: Card = Card {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string(),
            list_id: list_id,
            description: "".to_string(),
            due_date_instant_seconds: 0,
            due_complete: false,
            label_ids: vec![],
            checklists_ids: vec![]
        };

        let _insert_result = cards_collection.insert_one(card.clone(), None).await?;
        Ok(card)
    }

    async fn update_card(card: &Card) -> Result<Card, Box<dyn std::error::Error>> {
        Err(Box::new(NotImplError{}))
    }

    async fn get_card_comments(card_id: ID) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {
        let comments: Vec<CardComment>;

        unsafe {
            let comments_collection: Collection<CardComment> = db.clone().unwrap().collection::<CardComment>("comments");
            let cursor = comments_collection.find(doc! {
                "card_id": doc! {
                    "trello_id": card_id.trello_id.unwrap(),
                    "local_id": card_id.local_id.unwrap()
                }
            }, None).await?;
            comments = cursor.try_collect().await?;
        }

        Ok(comments)
    }

    async fn add_card_comment(card_id: ID, text: &str) -> Result<CardComment, Box<dyn std::error::Error>> {
        let comments_collection: Collection<CardComment>;
        unsafe {
            comments_collection = db.clone().unwrap().collection::<CardComment>("comments");
        }
        let object_id = oid::ObjectId::new();
        let comment: CardComment = CardComment {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            text: text.to_string(),
            card_id: card_id,
            commenter_name: "".to_string(), // TODO: Handle this better
            comment_time_instant_seconds: 0,
        };

        let _insert_result = comments_collection.insert_one(comment.clone(), None).await?;
        Ok(comment)
    }

    async fn get_card_checklists(card_id: ID) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>> {
        let checklists: Vec<CardChecklist>;

        unsafe {
            let checklists_collection: Collection<CardChecklist> = db.clone().unwrap().collection::<CardChecklist>("checklists");
            let cursor = checklists_collection.find(doc! {
                "card_id": doc! {
                    "trello_id": card_id.trello_id.unwrap(),
                    "local_id": card_id.local_id.unwrap()
                }
            }, None).await?;
            checklists = cursor.try_collect().await?;
        }

        Ok(checklists)
    }

    async fn create_card_checklist(card_id: ID, name: &str) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        let checklists_collection: Collection<CardChecklist>;
        unsafe {
            checklists_collection = db.clone().unwrap().collection::<CardChecklist>("checklists");
        }
        let object_id = oid::ObjectId::new();
        let checklist: CardChecklist = CardChecklist {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string(),
            card_id: card_id,
        };

        let _insert_result = checklists_collection.insert_one(checklist.clone(), None).await?;
        Ok(checklist)
    }

    async fn get_checklist_tasks(checklist_id: ID) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>> {
        let tasks: Vec<CardChecklistTask>;

        unsafe {
            let tasks_collection: Collection<CardChecklistTask> = db.clone().unwrap().collection::<CardChecklistTask>("tasks");
            let cursor = tasks_collection.find(doc! {
                "checklist_id": doc! {
                    "trello_id": checklist_id.trello_id.unwrap(),
                    "local_id": checklist_id.local_id.unwrap()
                }
            }, None).await?;
            tasks = cursor.try_collect().await?;
        }

        Ok(tasks)
    }

    async fn create_checklist_task(checklist_id: ID, name: &str) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let tasks_collection: Collection<CardChecklistTask>;
        unsafe {
            tasks_collection = db.clone().unwrap().collection::<CardChecklistTask>("tasks");
        }
        let object_id = oid::ObjectId::new();
        let task: CardChecklistTask = CardChecklistTask {
            _id: ID {
                trello_id: None,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string(),
            checklist_id: checklist_id,
            is_complete: false
        };

        let _insert_result = tasks_collection.insert_one(task.clone(), None).await?;
        Ok(task)
    }

    async fn update_checklist_task(card_id: ID, task: &CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
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