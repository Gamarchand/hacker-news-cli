use std::error::Error;
use serde::Deserialize;


#[derive(thiserror::Error, Debug)]
pub enum HNewsApiError{
    #[error("Failed to fetch stories")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error)

}


#[derive(Deserialize, Debug)]
pub struct Articles {
    pub hits: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub created_at: String,
    pub title: String,
    pub url: Option<String>,
    pub objectID: String

}

pub fn get_articles(url: &str) -> Result<Articles, HNewsApiError>{

    let response = ureq::get(url).call().map_err(|e| HNewsApiError::RequestFailed(e))
    ?.into_string().map_err(|e| HNewsApiError::FailedResponseToString(e))?;

    let articles: Articles = serde_json::from_str(&response).map_err(|e| HNewsApiError::ArticleParseFailed(e))?;
        
    Ok(articles)

}

pub enum Endpoint{
    FrontPage
}

pub enum Country{
    Us
}

struct HNewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country


}

impl HNewsAPI{
    fn new(api_key: &str) -> HNewsAPI{

        HNewsAPI{
            api_key: api_key.to_string(),
            endpoint: Endpoint::FrontPage,
            country: Country::Us
        }

    }

    fn endpoint(&mut self, endpoint: Endpoint) -> &mut HNewsAPI {
        self.endpoint = endpoint;

    }

    fn country(&mut self, country: Country) -> &mut HNewsAPI {
        self.country = country;

    }
}
