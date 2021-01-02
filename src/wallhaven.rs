// Downloading from wallhaven api
use attohttpc;
use serde_json;
use std::error::Error;
use serde::Deserialize;
use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct Page {
    pub data: Vec<Data>,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub id: String,
    pub url: String,
    pub views: u32,
    pub favorites: u32,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: u32,
    pub dimension_y: u32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u32,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
}

#[derive(Debug, Deserialize)]
pub struct Thumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub current_page: u32,
    pub last_page: u32,
    pub per_page: u32,
    pub total: u32,
    pub query: Option<String>,
    pub seed: Option<String>,
}

const API_URL: &str = "https://wallhaven.cc/api/v1/";

pub fn get_search_page(config: &Config, page_num: u32) -> Result<Page, Box<dyn Error>> {
    let search_url = format!("{}search", API_URL);
    let response = attohttpc::get(search_url).send()?;
    let response_str = response.text()?;
    //println!("Response: {}", response_str);
    let page: Page = serde_json::from_str(&response_str)?;
    Ok(page)
}
