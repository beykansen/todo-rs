use mongodb::{Client, options::ClientOptions, options::FindOptions, error::Error, Collection, };
use bson::{doc, Document};
use crate::model::Todo;
use crate::settings::Settings;
use futures::stream::StreamExt;

#[derive(Clone)]
pub struct Repository {
   db_name : String,
   collection_name : String,
   url: String
}

impl Repository {
    pub fn new(settings : &Settings) ->  Self {
        Repository {
            db_name: settings.database.db_name.to_owned(),
            url: settings.database.url.to_owned(),
            collection_name: settings.database.collection_name.to_owned()
        }
    }

    async fn get_collection(&self) ->  Result<Collection, Error> {
        let mut client_options = ClientOptions::parse(self.url.as_str()).await?;
        client_options.app_name = Some("todo-rs".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database(self.db_name.as_str());
        let collection = db.collection(self.collection_name.as_str());

        Ok(collection)
    }

    pub async fn insert(&self, todo: Todo) ->  Result<bool, Error> {
        let serialized_todo = bson::to_bson(&todo)?;
        let document = serialized_todo.as_document().unwrap();

        let collection = self.get_collection().await?;
        let insert_result = collection.insert_one(document.to_owned(), None).await?;
        insert_result
            .inserted_id
            .as_object_id().expect("Retrieved _id should have been of type ObjectId");

        Ok(true)
    }

    pub async fn get(&self, id : &str) ->  Result<Option<Todo>, Error> {
        let collection = self.get_collection().await?;
        let result = collection.find_one(doc! {"id": id}, None).await?;
        match result {
            Some(doc) => {
                Ok(Some(bson::from_bson(bson::Bson::Document(doc)).unwrap()))
            }
            None => {
                Ok(None)
            }
        }
    }

    pub async fn get_all(&self, done : Option<bool>) ->  Result<Vec<Todo>, Error> {
        let collection = self.get_collection().await?;
        let mut filter: Option<Document> = None;
        let find_options = FindOptions::builder().sort(doc! { "added_at": -1 }).build();
        match done {
            Some(d) => {
                filter = Some(doc! { "done": d })
            }
            _ => (),
        }
        let mut cursor = collection.find(filter, find_options).await?;
        let mut todos : Vec<Todo> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                   todos.push(bson::from_bson(bson::Bson::Document(doc)).unwrap());
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(todos)
    }

    pub async fn update(&self, id : &str, update : bson::Document) ->  Result<bool, Error> {
        let filter = doc! {"id":id};
        let collection = self.get_collection().await?;
        let update_result = collection.update_one(filter, update, None).await?;
        if update_result.matched_count >0 && update_result.modified_count > 0 {
            return Ok(true)
        }
        Ok(false)
    }

    pub async fn delete(&self, id : &str) ->  Result<bool, Error> {
        let filter = doc! {"id":id};
        let collection = self.get_collection().await?;
        let delete_result = collection.delete_one(filter, None).await?;
        if delete_result.deleted_count > 0 {
            return Ok(true)
        }
        Ok(false)
    }
}