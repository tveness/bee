use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WordMap(pub HashMap<String, Vec<String>>);

pub fn load_sorted_words() -> Result<WordMap> {
    let sorted_words_bytes_compressed = include_bytes!("../sowpods_sorted.postcard.miniz");
    let sorted_words_bytes =
        miniz_oxide::inflate::decompress_to_vec(sorted_words_bytes_compressed).unwrap();
    let sorted_words: WordMap = postcard::from_bytes(&sorted_words_bytes).unwrap();

    Ok(sorted_words)
}
