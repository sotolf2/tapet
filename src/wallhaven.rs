// Downloading from wallhaven api
use attohttpc;
use serde_json;
use std::error::Error;
use serde::Deserialize;
use crate::config::Config;
use crate::config;
use crate::core;

#[derive(Debug, Deserialize)]
struct Page {
    data: Vec<Data>,
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Data {
    id: String,
    url: String,
    views: u32,
    favorites: u32,
    source: String,
    purity: String,
    category: String,
    dimension_x: u32,
    dimension_y: u32,
    resolution: String,
    ratio: String,
    file_size: u32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    path: String,
    thumbs: Thumbs,
}

#[derive(Debug, Deserialize)]
struct Thumbs {
    large: String,
    original: String,
    small: String,
}

#[derive(Debug, Deserialize)]
struct Meta {
    current_page: u32,
    last_page: u32,
    per_page: u32,
    total: u32,
    query: Option<String>,
    seed: Option<String>,
}

const API_URL: &str = "https://wallhaven.cc/api/v1/";

fn get_search_page(config: &Config, page_num: u32) -> Result<Page, Box<dyn Error>> {
    let tags = &config.wallhaven.tags;
    let search_url = format!("{}search", API_URL);
    let response = attohttpc::get(search_url)
        .param("q", tags)
        .param("page", page_num)
        .send()?;
    let response_str = response.text()?;
    //println!("Response: {}", response_str);
    let page: Page = serde_json::from_str(&response_str)?;
    Ok(page)
}

fn page_to_urls(page: Page) -> Vec<String> {
    let mut urls = Vec::new();

    for data in page.data {
        urls.push(data.path);
    }

    urls
}

pub fn download_images(config: &Config, history_path: &str) -> Result<(), Box<dyn Error>> {
    let num_to_keep = config.wallhaven.download_number;
    let current_num = core::number_downloaded(config)?;
    let to_download = num_to_keep - current_num;

    // the wallhaven api gives us 24 urls per search page.
    // so we need to download enough pages to fill up our request.
   let mut page_num = 1;
   let history = config::retrive_history(history_path)?.urls;
   let mut urls: Vec<String> = Vec::new();

   while urls.len() <= to_download as usize {
        let page = get_search_page(config, page_num)?;
        
        for url in page_to_urls(page) {
            if !history.contains(&url) {
               urls.push(url);
            }
        }
        page_num += 1;
   }

   let download_queue: Vec<&String> = urls.iter().take(to_download as usize).collect();


   for url in &download_queue {
       println!("Downloading: {}", url);
       core::download_image(config ,&url)?;
   }

   config::append_history(config, history_path, download_queue)?;
    

    Ok(())
}

