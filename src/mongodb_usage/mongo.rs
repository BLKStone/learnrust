
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;


#[allow(dead_code)]
pub fn test_insert() {
    // Direct connection to a server. Will not look for other servers in the topology.
    let client = Client::connect("localhost", 27017).ok().expect("Failed to initialize client.");
    let coll = client.db("media").collection("movies");

    let film_a = doc!{ "title" => "Ferris Bueller’s Day Off" };
    let film_b = doc!{ "title" => "Airplane!" };

    // Insert multiple documents with default options.
    coll.insert_many(vec![film_a, film_b], None).ok().expect("Failed to insert documents.");

    coll.insert_one(doc!{ "title" => "Inside Job" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "Margin Call" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "Too Big to Fall" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "The Big Short" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "The Wolf of Wall Street" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "Enron: The Smartest Guys in the Room" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "The Infiltrator"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "The Informant!"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "Casino Jack"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "The New Detectives: Case Studies in Forensic Science"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "Le monde selon Monsanto"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "The Stanford Prison Experiment"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "Kill the Messenger"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "中国警方在行动"}, None).unwrap();
    coll.insert_one(doc!{ "title" => "辩护人"}, None).unwrap();

}
#[allow(dead_code)]
pub fn test_traverse_collection(){
    // Direct connection to a server. Will not look for other servers in the topology.
    let client = Client::connect("localhost", 27017).ok().expect("Failed to initialize client.");
    let coll = client.db("media").collection("movies");

    let mut cursor = coll.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref title)) = item.get("title") {
                println!("title: {}", title);
            }
        }
    }


}

#[allow(dead_code)]
pub fn test() {
    // Direct connection to a server. Will not look for other servers in the topology.
    let client = Client::connect("localhost", 27017).ok().expect("Failed to initialize client.");
    let coll = client.db("media").collection("movies");

    coll.insert_one(doc!{ "title" => "Inside Job" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "Margin Call" }, None).unwrap();
    coll.insert_one(doc!{ "title" => "Too Big to Fall" }, None).unwrap();

    coll.update_one(doc!{}, doc!{ "director" => "Robert Zemeckis" }, None).unwrap();
    // coll.delete_many(doc!{}, None).unwrap();



}

pub fn main() {

    test()

}
