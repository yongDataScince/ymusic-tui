use std::str::FromStr;

use reqwest::Url;
use crate::types::SearchType;

pub fn search_uri(text: String, r#type: Option<SearchType>, page: Option<u32>) -> Result<Url, ()> {
  let base_search_uri = "https://music.yandex.ru/search";
  
  let mut result_uri = format!("{base_search_uri}?text={text}");

  if r#type.is_some() {
    result_uri = format!("{}?type={}", result_uri.clone(), r#type.unwrap().to_string())
  }
  if page.is_some() {
    result_uri = format!("{}?page={}", result_uri.clone(), page.unwrap().to_string())
  } else {
    result_uri = format!("{}?page=1", result_uri.clone())
  }

  Url::from_str(&result_uri).map_err(|e| {
    eprintln!("Error in parse search URL: {e}");
  })
}