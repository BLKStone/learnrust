#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

mod std_library_types;
mod mongodb_usage;

fn main() {

    // std_library_types::vectors::main()
    mongodb_usage::mongo::main()
    

}
