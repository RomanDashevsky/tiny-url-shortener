use mongodb::bson::{doc};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct UrlService {
    collection: Collection<Url>,
}

#[derive(Deserialize)]
pub struct InsertUrlDto {
    pub url: String,
}

#[derive(Serialize)]
pub struct CreatedUrlDto {
    pub key: String,
    pub url: String,
    pub created: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url {
    pub key: String,
    pub url: String,
}

impl UrlService {
    pub fn new(collection: Collection<Url>) -> UrlService {
        UrlService { collection }
    }

    pub async fn create(&self, url: &str) -> Result<Option<CreatedUrlDto>, Error> {
        let mut has_url = false;
        let found_result = self.get_founded_url(url).await;
        match &found_result {
            Ok(document) => {
                if document.is_some() {
                    has_url = true;
                }
            },
            Err(_) => ()
        }

        if has_url {
            return found_result;
        }

        self.get_created_url(url).await
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<Url>, Error> {
        self.collection.find_one(doc! { "key": key }, None).await
    }

    pub async fn find_by_url(&self, url: &str) -> Result<Option<Url>, Error> {
        self.collection.find_one(doc! { "url": url }, None).await
    }

    async fn get_founded_url(&self, url: &str) -> Result<Option<CreatedUrlDto>, Error> {
        let found_result = self.find_by_url(url).await;

        found_result.map(|document| {
            let mut created_url_dto: Option<CreatedUrlDto> = None;

            if document.is_some() {
                let url_data: Url  = document.unwrap();
                created_url_dto = Some(CreatedUrlDto {
                    key: url_data.key,
                    url: url_data.url,
                    created: false,
                })
            }
            created_url_dto
        })
    }

    async fn get_created_url(&self, url: &str) -> Result<Option<CreatedUrlDto>, Error> {
        // TODO: implement generate new key logic
        let key = "Afgr2";

        let raw_result = self.collection
            .insert_one(Url {
                url: url.to_string(),
                key: key.to_string()
            }, None)
            .await;

        raw_result.map(|_| {
            Some(CreatedUrlDto {
                key: key.to_string(),
                url: url.to_string(),
                created: true,
            })
        })
    }
}