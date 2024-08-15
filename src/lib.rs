use anyhow::Result;
use miniz_oxide::inflate::decompress_to_vec;
use postcard::from_bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WordMap(pub HashMap<String, Vec<String>>);

pub fn load_sorted_words() -> Result<WordMap> {
    let sorted_words_bytes_compressed = include_bytes!("../sowpods_sorted.postcard.miniz");
    let sorted_words_bytes = decompress_to_vec(sorted_words_bytes_compressed).unwrap();
    let sorted_words: WordMap = from_bytes(&sorted_words_bytes).unwrap();

    Ok(sorted_words)
}
