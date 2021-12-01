use crate::data::data_repository::DataStore;
use crate::data::*;

use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc, options::ClientOptions, options::InsertManyOptions, options::UpdateOptions, Client,
    Collection, Database, bson::oid, bson::Document, options::FindOneAndUpdateOptions
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
                let _update_result = boards_collection.update_one(
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
                let _insert_result = boards_collection.insert_one(board.clone(), None).await?;
            }

            boards_with_ids.push(board);
        }

        Ok(boards_with_ids)
    }

    pub async fn sync_labels(board_id: ID, trello_labels: Vec<CardLabel>) -> Result<Vec<CardLabel>, Box<dyn std::error::Error>> {
        let labels_collection: Collection<CardLabel>;

        unsafe {
            labels_collection = db.clone().unwrap().collection::<CardLabel>("labels");
        }

        let mut labels_with_ids: Vec<CardLabel> = vec![];
        let existing_labels: Vec<CardLabel> = MongoDataStore::get_all_board_labels(board_id).await?;

        let mut existing_label_by_trello_id: HashMap<String, CardLabel> = HashMap::new();

        for label in existing_labels {
            existing_label_by_trello_id.insert(label.clone()._id.trello_id.unwrap(), label);
        }

        for mut label in trello_labels {
            let existing_label = existing_label_by_trello_id.get(&label._id.trello_id.clone().unwrap());
            if existing_label.is_some() {
                label._id.local_id.replace(existing_label.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = label.to_doc::<CardLabel>(true);
                let _update_result = labels_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": label._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                label._id.local_id.replace(object_id.to_hex());
                let _insert_result = labels_collection.insert_one(label.clone(), None).await?;
            }

            labels_with_ids.push(label);
        }

        Ok(labels_with_ids)
    }

    pub async fn sync_lists(board_id: ID, trello_lists: Vec<BoardList>) -> Result<Vec<BoardList>, Box<dyn std::error::Error>> {
        let lists_collection: Collection<BoardList>;

        unsafe {
            lists_collection = db.clone().unwrap().collection::<BoardList>("lists");
        }

        let mut lists_with_ids: Vec<BoardList> = vec![];
        let existing_lists: Vec<BoardList> = MongoDataStore::get_all_board_lists(board_id).await?;

        let mut existing_list_by_trello_id: HashMap<String, BoardList> = HashMap::new();

        for list in existing_lists {
            existing_list_by_trello_id.insert(list.clone()._id.trello_id.unwrap(), list);
        }

        for mut list in trello_lists {
            let existing_list = existing_list_by_trello_id.get(&list._id.trello_id.clone().unwrap());
            if existing_list.is_some() {
                list._id.local_id.replace(existing_list.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = list.to_doc::<BoardList>(true);
                let _update_result = lists_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": list._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                list._id.local_id.replace(object_id.to_hex());
                let _insert_result = lists_collection.insert_one(list.clone(), None).await?;
            }

            lists_with_ids.push(list);
        }

        Ok(lists_with_ids)
    }

    pub async fn sync_cards(list_id: ID, trello_cards: Vec<Card>) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        // TODO: Please refactor this, there is lots of almost-duplicate code
        let cards_collection: Collection<Card>;

        unsafe {
            cards_collection = db.clone().unwrap().collection::<Card>("cards");
        }

        let mut cards_with_ids: Vec<Card> = vec![];
        let existing_cards: Vec<Card> = MongoDataStore::get_all_list_cards(list_id).await?;

        let mut existing_card_by_trello_id: HashMap<String, Card> = HashMap::new();

        for card in existing_cards {
            existing_card_by_trello_id.insert(card.clone()._id.trello_id.unwrap(), card);
        }

        for mut card in trello_cards {
            let existing_card = existing_card_by_trello_id.get(&card._id.trello_id.clone().unwrap());
            if existing_card.is_some() {
                card._id.local_id.replace(existing_card.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = card.to_doc::<Card>(true);
                let _update_result = cards_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": card._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                card._id.local_id.replace(object_id.to_hex());
                let _insert_result = cards_collection.insert_one(card.clone(), None).await?;
            }

            cards_with_ids.push(card);
        }

        Ok(cards_with_ids)
    }

    pub async fn sync_checklists(card_id: ID, trello_checklists: Vec<CardChecklist>) -> Result<Vec<CardChecklist>, Box<dyn std::error::Error>> {
        let checklists_collection: Collection<CardChecklist>;

        unsafe {
            checklists_collection = db.clone().unwrap().collection::<CardChecklist>("checklists");
        }

        let mut checklists_with_ids: Vec<CardChecklist> = vec![];
        let existing_checklists: Vec<CardChecklist> = MongoDataStore::get_card_checklists(card_id).await?;

        let mut existing_checklist_by_trello_id: HashMap<String, CardChecklist> = HashMap::new();

        for checklist in existing_checklists {
            existing_checklist_by_trello_id.insert(checklist.clone()._id.trello_id.unwrap(), checklist);
        }

        for mut checklist in trello_checklists {
            let existing_checklist = existing_checklist_by_trello_id.get(&checklist._id.trello_id.clone().unwrap());
            if existing_checklist.is_some() {
                checklist._id.local_id.replace(existing_checklist.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = checklist.to_doc::<CardChecklist>(true);
                let _update_result = checklists_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": checklist._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                checklist._id.local_id.replace(object_id.to_hex());
                let _insert_result = checklists_collection.insert_one(checklist.clone(), None).await?;
            }

            checklists_with_ids.push(checklist);
        }

        Ok(checklists_with_ids)
    }

    pub async fn sync_tasks(checklist_id: ID, trello_tasks: Vec<CardChecklistTask>) -> Result<Vec<CardChecklistTask>, Box<dyn std::error::Error>> {
        let tasks_collection: Collection<CardChecklistTask>;

        unsafe {
            tasks_collection = db.clone().unwrap().collection::<CardChecklistTask>("tasks");
        }

        let mut tasks_with_ids: Vec<CardChecklistTask> = vec![];
        let existing_tasks: Vec<CardChecklistTask> = MongoDataStore::get_checklist_tasks(checklist_id).await?;

        let mut existing_task_by_trello_id: HashMap<String, CardChecklistTask> = HashMap::new();

        for task in existing_tasks {
            existing_task_by_trello_id.insert(task.clone()._id.trello_id.unwrap(), task);
        }

        for mut task in trello_tasks {
            let existing_task = existing_task_by_trello_id.get(&task._id.trello_id.clone().unwrap());
            if existing_task.is_some() {
                task._id.local_id.replace(existing_task.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = task.to_doc::<CardChecklistTask>(true);
                let _update_result = tasks_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": task._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                task._id.local_id.replace(object_id.to_hex());
                let _insert_result = tasks_collection.insert_one(task.clone(), None).await?;
            }

            tasks_with_ids.push(task);
        }

        Ok(tasks_with_ids)
    }

    pub async fn sync_comments(card_id: ID, trello_comments: Vec<CardComment>) -> Result<Vec<CardComment>, Box<dyn std::error::Error>> {
        let comments_collection: Collection<CardComment>;

        unsafe {
            comments_collection = db.clone().unwrap().collection::<CardComment>("comments");
        }

        let mut comments_with_ids: Vec<CardComment> = vec![];
        let existing_comments: Vec<CardComment> = MongoDataStore::get_card_comments(card_id).await?;

        let mut existing_comment_by_trello_id: HashMap<String, CardComment> = HashMap::new();

        for comment in existing_comments {
            existing_comment_by_trello_id.insert(comment.clone()._id.trello_id.unwrap(), comment);
        }

        for mut comment in trello_comments {
            let existing_comment = existing_comment_by_trello_id.get(&comment._id.trello_id.clone().unwrap());
            if existing_comment.is_some() {
                comment._id.local_id.replace(existing_comment.unwrap()._id.local_id.as_ref().unwrap().to_string());
                let update_doc = comment.to_doc::<CardComment>(true);
                let _update_result = comments_collection.update_one(
                    doc! {
                        "_id": doc! {
                            "trello_id": comment._id.trello_id.clone().unwrap()
                        },
                    },
                    update_doc,
                    None
                ).await?;
            } else {
                let object_id = oid::ObjectId::new();
                comment._id.local_id.replace(object_id.to_hex());
                let _insert_result = comments_collection.insert_one(comment.clone(), None).await?;
            }

            comments_with_ids.push(comment);
        }

        Ok(comments_with_ids)
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

    async fn create_board(name: &str, trello_id: Option<String>) -> Result<Board, Box<dyn std::error::Error>> {
        let boards_collection: Collection<Board>;
        unsafe {
            boards_collection = db.clone().unwrap().collection::<Board>("boards");
        }
        let object_id = oid::ObjectId::new();
        let board: Board = Board {
            _id: ID {
                trello_id: trello_id,
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
        unsafe {
            let labels_collection: Collection<CardLabel> = db.clone().unwrap().collection::<CardLabel>("labels");
            let _delete_result = labels_collection.delete_one(doc! {
                "_id": doc! {
                    "trello_id": label_id.trello_id.unwrap(),
                    "local_id": label_id.local_id.unwrap()
                }
            }, None).await?;
        }
        
        Ok(())
    }

    async fn update_board_label(label_id: ID, name: &str, color: &str) -> Result<CardLabel, Box<dyn std::error::Error>> {
        unsafe {
            let labels_collection = db.clone().unwrap().collection::<CardLabel>("labels");
            let update_doc = doc! {
                "$set": doc! {
                    "name": name.to_string(),
                    "color": color.to_string()
                }
            };

            let find_update_options = FindOneAndUpdateOptions::builder()
                .return_document(mongodb::options::ReturnDocument::After)
                .upsert(Some(true))
                .build();
            let find_update_result = labels_collection.find_one_and_update( doc! {
                "_id": label_id.to_doc::<ID>(false)
            }, update_doc, find_update_options).await?;
            
            Ok(find_update_result.unwrap())
        }
    }

    async fn create_board_label(board_id: ID, name: &str, color: &str, trello_id: Option<String>) -> Result<CardLabel, Box<dyn std::error::Error>> {
        let labels_collection: Collection<CardLabel>;
        unsafe {
            labels_collection = db.clone().unwrap().collection::<CardLabel>("labels");
        }
        let object_id = oid::ObjectId::new();
        let label: CardLabel = CardLabel {
            _id: ID {
                trello_id: trello_id,
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

    async fn create_board_list(board_id: ID, name: &str, trello_id: Option<String>) -> Result<BoardList, Box<dyn std::error::Error>> {
        let lists_collection: Collection<BoardList>;
        unsafe {
            lists_collection = db.clone().unwrap().collection::<BoardList>("lists");
        }
        let object_id = oid::ObjectId::new();
        let list: BoardList = BoardList {
            _id: ID {
                trello_id: trello_id,
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

    async fn create_list_card(list_id: ID, name: &str, trello_id: Option<String>) -> Result<Card, Box<dyn std::error::Error>> {
        let cards_collection: Collection<Card>;
        unsafe {
            cards_collection = db.clone().unwrap().collection::<Card>("cards");
        }
        let object_id = oid::ObjectId::new();
        let card: Card = Card {
            _id: ID {
                trello_id: trello_id,
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
        unsafe {
            let cards_collection = db.clone().unwrap().collection::<Card>("cards");
            let update_doc = card.to_doc::<Card>(true);

            let find_update_options = FindOneAndUpdateOptions::builder()
                .return_document(mongodb::options::ReturnDocument::After)
                .upsert(Some(true))
                .build();
            let find_update_result = cards_collection.find_one_and_update( doc! {
                "_id": card._id.to_doc::<ID>(false)
            }, update_doc, find_update_options).await?;
            
            Ok(find_update_result.unwrap())
        }
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

    async fn add_card_comment(card_id: ID, text: &str, trello_id: Option<String>) -> Result<CardComment, Box<dyn std::error::Error>> {
        let comments_collection: Collection<CardComment>;
        unsafe {
            comments_collection = db.clone().unwrap().collection::<CardComment>("comments");
        }
        let object_id = oid::ObjectId::new();
        let comment: CardComment = CardComment {
            _id: ID {
                trello_id: trello_id,
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

    async fn create_card_checklist(card_id: ID, name: &str, trello_id: Option<String>) -> Result<CardChecklist, Box<dyn std::error::Error>> {
        let checklists_collection: Collection<CardChecklist>;
        unsafe {
            checklists_collection = db.clone().unwrap().collection::<CardChecklist>("checklists");
        }
        let object_id = oid::ObjectId::new();
        let checklist: CardChecklist = CardChecklist {
            _id: ID {
                trello_id: trello_id,
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

    async fn create_checklist_task(checklist_id: ID, name: &str, trello_id: Option<String>) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        let tasks_collection: Collection<CardChecklistTask>;
        unsafe {
            tasks_collection = db.clone().unwrap().collection::<CardChecklistTask>("tasks");
        }
        let object_id = oid::ObjectId::new();
        let task: CardChecklistTask = CardChecklistTask {
            _id: ID {
                trello_id: trello_id,
                local_id: Some(object_id.to_hex())
            },
            name: name.to_string(),
            checklist_id: checklist_id,
            is_complete: false
        };

        let _insert_result = tasks_collection.insert_one(task.clone(), None).await?;
        Ok(task)
    }

    async fn update_checklist_task(_card_id: ID, task: &CardChecklistTask) -> Result<CardChecklistTask, Box<dyn std::error::Error>> {
        unsafe {
            let tasks_collection = db.clone().unwrap().collection::<CardChecklistTask>("tasks");
            let update_doc = task.to_doc::<CardChecklistTask>(true);

            let find_update_options = FindOneAndUpdateOptions::builder()
                .return_document(mongodb::options::ReturnDocument::After)
                .upsert(Some(true))
                .build();
            let find_update_result = tasks_collection.find_one_and_update( doc! {
                "_id": task._id.to_doc::<ID>(false)
            }, update_doc, find_update_options).await?;
            
            Ok(find_update_result.unwrap())
        }
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

impl ToDocument for ID {
    fn to_doc<ID>(&self, is_update_op: bool) -> Document {
        return doc! {
            "trello_id": self.trello_id.clone().unwrap(),
            "local_id": self.local_id.clone().unwrap()
        }
    }
}

impl ToDocument for BoardList {
    fn to_doc<BoardList>(&self, is_update_op: bool) -> Document {
        return if is_update_op {
            doc! {
                "$set": doc! {
                    "board_id": self.board_id.to_doc::<ID>(false),
                    "name": self.name.clone(),
                }
            }
        } else {
            doc! {
                "_id": self._id.to_doc::<ID>(false),
                "board_id": self.board_id.to_doc::<ID>(false),
                "name": self.name.clone(),
            }
        }
    }
}

impl ToDocument for CardLabel {
    fn to_doc<CardLabel>(&self, is_update_op: bool) -> Document {
        return if is_update_op {
            doc! {
                "$set": doc! {
                    "board_id": self.board_id.to_doc::<ID>(false),
                    "name": self.name.clone(),
                    "color": self.color.clone()
                }
            }
        } else {
            doc! {
                "_id": self._id.to_doc::<ID>(false),
                "board_id": self.board_id.to_doc::<ID>(false),
                "name": self.name.clone(),
                "color": self.color.clone()
            }
        }
    }
}

impl ToDocument for CardChecklist {
    fn to_doc<CardChecklist>(&self, is_update_op: bool) -> Document {
        return if is_update_op {
            doc! {
                "$set": doc! {
                    "card_id": self.card_id.to_doc::<ID>(false),
                    "name": self.name.clone(),
                }
            }
        } else {
            doc! {
                "_id": self._id.to_doc::<ID>(false),
                "card_id": self.card_id.to_doc::<ID>(false),
                "name": self.name.clone(),
            }
        }
    }
}

impl ToDocument for CardChecklistTask {
    fn to_doc<CardChecklistTask>(&self, is_update_op: bool) -> Document {
        return if is_update_op {
            doc! {
                "$set": doc! {
                    "name": self.name.clone(),
                    "is_complete": self.is_complete,
                    "checklist_id": self.checklist_id.to_doc::<ID>(false)
                }
            }
        } else {
            doc! {
                "_id": self._id.to_doc::<ID>(false),
                "name": self.name.clone(),
                "is_complete": self.is_complete,
                "checklist_id": self.checklist_id.to_doc::<ID>(false)
            }
        }
    }
}

impl ToDocument for Card {
    fn to_doc<Card>(&self, is_update_op: bool) -> Document {
        let mut label_id_docs: Vec<Document> = vec![];
        let mut checklist_id_docs: Vec<Document> = vec!();

        for label_id in &self.label_ids {
            let doc = label_id.to_doc::<ID>(false);
            label_id_docs.push(doc);
        }

        for checklist_id in &self.checklists_ids {
            let doc = checklist_id.to_doc::<ID>(false);
            checklist_id_docs.push(doc);
        }

        return if is_update_op {
            doc! {
                "$set": doc! {
                    "name": self.name.clone(),
                    "description": self.description.clone(),
                    "due_date_instant_seconds": self.due_date_instant_seconds,
                    "due_complete": self.due_complete,
                    "label_ids": label_id_docs,
                    "checklists_ids": checklist_id_docs,
                    "list_id": self.list_id.to_doc::<ID>(false)
                }
            }
        } else {
            doc! {
                "_id": self._id.to_doc::<ID>(false),
                "name": self.name.clone(),
                "description": self.description.clone(),
                "due_date_instant_seconds": self.due_date_instant_seconds,
                "due_complete": self.due_complete,
                "label_ids": label_id_docs,
                "checklists_ids": checklist_id_docs,
                "list_id": self.list_id.to_doc::<ID>(false)
            }
        }
    }
}

impl ToDocument for CardComment {
    fn to_doc<CardComment>(&self, is_update_op: bool) -> Document {
        let mut label_id_docs: Vec<Document> = vec![];
        let mut checklist_id_docs: Vec<Document> = vec!();

        return if is_update_op {
            doc! {
                "$set": doc! {
                    "commenter_name": self.commenter_name.clone(),
                    "text": self.text.clone(),
                    "comment_time_instant_seconds": self.comment_time_instant_seconds,
                    "card_id": self.card_id.to_doc::<ID>(false)
                }
            }
        } else {
            doc! {
                "_id": self._id.to_doc::<ID>(false),
                "commenter_name": self.commenter_name.clone(),
                "text": self.text.clone(),
                "comment_time_instant_seconds": self.comment_time_instant_seconds,
                "card_id": self.card_id.to_doc::<ID>(false)
            }
        }
    }
}