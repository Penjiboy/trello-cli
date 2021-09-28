#[macro_use]
extern crate lazy_static;

use mongodb::{Client, options::ClientOptions };
use structopt::StructOpt;
use tokio;

use std::path::PathBuf;

mod data;
mod service;
mod control;

#[derive(Debug, StructOpt)]
struct CliArgs {
    #[structopt(short, long)]
    interactive: bool,

    #[structopt(short = "t", long = "token", parse(from_os_str))]
    token_file: Option<PathBuf>,

    #[structopt(short = "k", long = "key", parse(from_os_str))]
    key_file: Option<PathBuf>
}

#[tokio::main]
async fn main() {
    let args = CliArgs::from_args();

    if args.interactive {
        // Do something
        control::interactive_cli::run().await;
        return
    }

    // let mongo_result = test_mongo_connection().await;
    // assert_eq!(mongo_result.is_ok(), true);
    // println!();

}

async fn test_mongo_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a connection string into an options struct
    let mut client_options = ClientOptions::parse("mongodb://root:rootpassword@localhost:32392").await?;

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
    let db = client.database("trelloData");

    // List the names of the collections in that database
    println!("Collections in trelloData: ");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}