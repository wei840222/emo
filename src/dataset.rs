use anyhow::{Context, Result};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const EMBEDDED_DATASET: &str = include_str!("../assets/Emoji_Sentiment_Data_v1.0.csv");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiData {
    pub emoji: String,
    pub codepoint: String,
    pub occurrences: u64,
    pub negative: u64,
    pub neutral: u64,
    pub positive: u64,
    pub name: String,
    pub block: String,
}

impl EmojiData {
    pub fn samples(&self) -> u64 {
        self.negative + self.neutral + self.positive
    }

    pub fn score(&self) -> f64 {
        let total = self.samples();
        if total == 0 {
            0.0
        } else {
            (self.positive as f64 - self.negative as f64) / (total as f64)
        }
    }

    pub fn intensity(&self) -> f64 {
        let total = self.samples();
        if total == 0 {
            0.0
        } else {
            (self.positive as f64 + self.negative as f64) / (total as f64)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EmojiDataset {
    by_char: HashMap<String, EmojiData>,
    by_codepoint: HashMap<String, EmojiData>,
}

impl EmojiDataset {
    pub fn load_embedded() -> Result<Self> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(EMBEDDED_DATASET.as_bytes());

        let mut by_char = HashMap::new();
        let mut by_codepoint = HashMap::new();

        for result in rdr.records() {
            let record = result.context("Failed to parse CSV record")?;
            let emoji_char = record.get(0).unwrap_or("").trim().to_string();
            let codepoint = record.get(1).unwrap_or("").trim().to_string();
            let occurrences = record.get(2).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let negative = record.get(4).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let neutral = record.get(5).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let positive = record.get(6).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let name = record.get(7).unwrap_or("").trim().to_string();
            let block = record.get(8).unwrap_or("").trim().to_string();

            let data = EmojiData {
                emoji: emoji_char.clone(),
                codepoint: codepoint.clone(),
                occurrences,
                negative,
                neutral,
                positive,
                name,
                block,
            };

            by_char.insert(emoji_char, data.clone());
            by_codepoint.insert(codepoint.to_lowercase(), data);
        }

        Ok(Self {
            by_char,
            by_codepoint,
        })
    }

    pub fn get_by_char(&self, emoji_char: &str) -> Option<&EmojiData> {
        self.by_char.get(emoji_char)
    }

    pub fn get_by_codepoint(&self, codepoint: &str) -> Option<&EmojiData> {
        self.by_codepoint.get(&codepoint.to_lowercase())
    }

    pub fn len(&self) -> usize {
        self.by_char.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_char.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_embedded_dataset() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        assert!(!dataset.is_empty());
        assert!(dataset.len() >= 900);

        // Check 😂
        let joy = dataset.get_by_char("😂").expect("😂 should exist");
        assert_eq!(joy.name, "FACE WITH TEARS OF JOY");
        assert!(joy.score() > 0.0);

        // Check 😭
        let cry = dataset.get_by_char("😭").expect("😭 should exist");
        assert_eq!(cry.name, "LOUDLY CRYING FACE");
        assert!(cry.score() < 0.0);
    }
}

