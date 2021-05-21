use mongodb::bson::{doc, Document};
use mongodb::{error::Error, results::InsertOneResult, Collection};

#[derive(Clone)]
pub struct UrlService {
    collection: Collection,
}

impl UrlService {
    pub fn new(collection: Collection) -> UrlService {
        UrlService { collection }
    }

    pub async fn create(&self, url: &str) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(doc! {"url": url}, None).await
    }

    pub async fn get(&self) -> Result<Option<Document>, mongodb::error::Error> {
        self.collection.find_one(doc! {}, None).await
    }
}