use mongodb::bson::{doc};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
use mongodb::results::DeleteResult;
use nanoid::nanoid;
use std::env;

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
    pub raw_url: String,
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

    pub async fn delete(&self, url: &str) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "url": url }, None).await
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
                    raw_url: url_data.url,
                    url: self.get_shorted_url(&url_data.key),
                    created: false,
                })
            }
            created_url_dto
        })
    }

    async fn get_created_url(&self, url: &str) -> Result<Option<CreatedUrlDto>, Error> {
        let key_result = self.get_unique_key().await;

        match key_result {
            Ok(key) => {
                let raw_result = self.collection
                    .insert_one(Url {
                        url: url.to_string(),
                        key: (*key).to_string()
                    }, None)
                    .await;

                raw_result.map(|_| {
                    Some(CreatedUrlDto {
                        raw_url: url.to_string(),
                        url: self.get_shorted_url(&key),
                        created: true,
                    })
                })
            },
            Err(err) => Result::Err(err)
        }
    }

    async fn get_unique_key(&self) -> Result<String, Error> {
        loop {
            let key = nanoid!(8);
            let mut has_unique_key = false;

            let key_result = self.find_by_key(&key).await
                .map(|url_data| {
                    if url_data.is_none() {
                        has_unique_key = true;
                    }
                    key
                });

            if has_unique_key {
                return key_result;
            }
        }

    }

    fn get_shorted_url(&self, key: &str) -> String {
        let host = env::var("PUBLIC_HOST").unwrap();
        let port = env::var("PORT").unwrap();
        let is_local_env = env::var("ENV_TYPE").unwrap() == "local";
        let port_sub = if is_local_env && !port.is_empty() { format!(":{0}", port) } else { "".to_string() };
        format!("http://{0}{1}/{2}", host, port_sub, key)
    }
}