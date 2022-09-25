use mongodb::bson::{doc, DateTime};
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use mongodb::options::{ClientOptions, FindOneAndUpdateOptions, ReturnDocument};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorDocument {
    pub _id: ObjectId,
    pub uuid: String,
    pub mac: String,
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub profileOwnerId: String,
    pub apiToken: String,
    pub createdAt: DateTime,
    pub modifiedAt: DateTime,
    pub value: f64,
}

#[tokio::main]
async fn main() {
    println!("starting up");

    async_global_executor::block_on(async {
        let mongo_uri = "mongodb://localhost:27017";
        let client_options = ClientOptions::parse(mongo_uri).await;
        let client = Client::with_options(client_options.unwrap()).unwrap();

        for i in 1..1000 {
            println!("Writing i = {}", i);

            let uuid = "4cbf8674-6f37-402b-8c35-a76c0f8965ca".to_string();
            let api_token = "473a4861-632b-4915-b01e-cf1d418966c6".to_string();
            let value = 10.0;

            let collection: Collection<SensorDocument> = client.database("testdb").collection::<SensorDocument>("testdb");

            let find_one_and_update_options = FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build();

            let sensor_doc: Option<SensorDocument> = collection
                .find_one_and_update(
                    doc! { "uuid": uuid,"apiToken": api_token },
                    doc! { "$set": {
                        "value": value,
                        "modifiedAt": DateTime::now()}
                    },
                    find_one_and_update_options,
                )
                .await.unwrap();

            match sensor_doc {
                Some(sensor_doc) => {
                    println!("update success {:?}", sensor_doc);
                }
                None => {
                    println!("none!!!!!");
                }
            };
        }
    })
}