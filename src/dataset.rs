use anyhow::{Context, Result};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const EMBEDDED_DATASET: &str = include_str!("../assets/Emoji_Sentiment_Data_v1.0.csv");
const EMBEDDED_EXTENDED_DATASET: &str = include_str!("../assets/EmojiNet_v1.0.csv");
const EMBEDDED_GOEMOTIONS_DATASET: &str = include_str!("../assets/GoEmotions_27_Mapping.csv");

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
    pub primary_emotion: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoEmotionCategory {
    pub category: String,
    pub id: u8,
    pub polarity: String,
    pub mapped_emojis: Vec<String>,
    pub description: String,
}

#[derive(Debug, Default, Clone)]
pub struct EmojiDataset {
    by_char: HashMap<String, EmojiData>,
    by_codepoint: HashMap<String, EmojiData>,
    goemotions_categories: HashMap<String, GoEmotionCategory>,
}

impl EmojiDataset {
    pub fn load_embedded() -> Result<Self> {
        let mut by_char = HashMap::new();
        let mut by_codepoint = HashMap::new();
        let mut goemotions_categories = HashMap::new();

        // 1. Load Primary Emoji Sentiment Ranking v1.0
        let mut rdr1 = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(EMBEDDED_DATASET.as_bytes());

        for result in rdr1.records() {
            let record = result.context("Failed to parse Emoji_Sentiment_Data CSV record")?;
            let emoji_char = record.get(0).unwrap_or("").trim().to_string();
            let codepoint = record.get(1).unwrap_or("").trim().to_string();
            let occurrences = record.get(2).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let negative = record.get(4).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let neutral = record.get(5).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let positive = record.get(6).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let name = record.get(7).unwrap_or("").trim().to_string();
            let block = record.get(8).unwrap_or("").trim().to_string();

            let primary_emotion = infer_goemotion(&name, positive, neutral, negative);

            let data = EmojiData {
                emoji: emoji_char.clone(),
                codepoint: codepoint.clone(),
                occurrences,
                negative,
                neutral,
                positive,
                name,
                block,
                primary_emotion,
            };

            by_char.insert(emoji_char.clone(), data.clone());
            by_codepoint.insert(codepoint.to_lowercase(), data);
        }

        // 2. Load Supplementary Extended Dataset (EmojiNet_v1.0.csv)
        let mut rdr2 = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(EMBEDDED_EXTENDED_DATASET.as_bytes());

        for result in rdr2.records() {
            let record = result.context("Failed to parse EmojiNet_v1.0 CSV record")?;
            let emoji_char = record.get(0).unwrap_or("").trim().to_string();
            if !by_char.contains_key(&emoji_char) {
                let codepoint = record.get(1).unwrap_or("").trim().to_string();
                let occurrences = record.get(2).unwrap_or("0").parse::<u64>().unwrap_or(0);
                let negative = record.get(3).unwrap_or("0").parse::<u64>().unwrap_or(0);
                let neutral = record.get(4).unwrap_or("0").parse::<u64>().unwrap_or(0);
                let positive = record.get(5).unwrap_or("0").parse::<u64>().unwrap_or(0);
                let name = record.get(6).unwrap_or("").trim().to_string();
                let block = record.get(7).unwrap_or("").trim().to_string();
                let primary_emotion = record.get(8).unwrap_or("Neutral").trim().to_string();

                let data = EmojiData {
                    emoji: emoji_char.clone(),
                    codepoint: codepoint.clone(),
                    occurrences,
                    negative,
                    neutral,
                    positive,
                    name,
                    block,
                    primary_emotion,
                };

                by_char.insert(emoji_char.clone(), data.clone());
                by_codepoint.insert(codepoint.to_lowercase(), data);
            }
        }

        // 3. Load GoEmotions 27 Category Mapping (GoEmotions_27_Mapping.csv)
        let mut rdr3 = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(EMBEDDED_GOEMOTIONS_DATASET.as_bytes());

        for result in rdr3.records() {
            let record = result.context("Failed to parse GoEmotions_27_Mapping CSV record")?;
            let category = record.get(0).unwrap_or("").trim().to_string();
            let id = record.get(1).unwrap_or("0").parse::<u8>().unwrap_or(0);
            let polarity = record.get(2).unwrap_or("Neutral").trim().to_string();
            let emojis_raw = record.get(3).unwrap_or("").trim();
            let description = record.get(4).unwrap_or("").trim().to_string();

            let mapped_emojis: Vec<String> = emojis_raw.split_whitespace().map(|s| s.to_string()).collect();

            let go_cat = GoEmotionCategory {
                category: category.clone(),
                id,
                polarity,
                mapped_emojis,
                description,
            };

            goemotions_categories.insert(category.to_lowercase(), go_cat);
        }

        Ok(Self {
            by_char,
            by_codepoint,
            goemotions_categories,
        })
    }

    pub fn get_by_char(&self, emoji_char: &str) -> Option<&EmojiData> {
        let clean = emoji_char.replace('\u{FE0F}', "");
        self.by_char
            .get(emoji_char)
            .or_else(|| self.by_char.get(&clean))
    }

    pub fn get_by_codepoint(&self, codepoint: &str) -> Option<&EmojiData> {
        self.by_codepoint.get(&codepoint.to_lowercase())
    }

    pub fn get_goemotion_category(&self, category: &str) -> Option<&GoEmotionCategory> {
        self.goemotions_categories.get(&category.to_lowercase())
    }

    pub fn len(&self) -> usize {
        self.by_char.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_char.is_empty()
    }
}

fn infer_goemotion(name: &str, pos: u64, neu: u64, neg: u64) -> String {
    let name_upper = name.to_uppercase();
    if name_upper.contains("LAUGH") || name_upper.contains("JOY") || name_upper.contains("TEARS OF JOY") {
        "Amusement".to_string()
    } else if name_upper.contains("LOVE") || name_upper.contains("HEART") || name_upper.contains("KISS") {
        "Love".to_string()
    } else if name_upper.contains("PARTY") || name_upper.contains("POPPER") || name_upper.contains("FIRE") || name_upper.contains("ROCKET") {
        "Excitement".to_string()
    } else if name_upper.contains("CRY") || name_upper.contains("SAD") || name_upper.contains("BROKEN") {
        "Sadness".to_string()
    } else if name_upper.contains("ANGRY") || name_upper.contains("RAGE") || name_upper.contains("DEVIL") {
        "Anger".to_string()
    } else if name_upper.contains("THINK") || name_upper.contains("QUESTION") {
        "Curiosity".to_string()
    } else if name_upper.contains("PRAY") || name_upper.contains("HANDS") || name_upper.contains("CLAP") {
        "Gratitude".to_string()
    } else if name_upper.contains("STAR") || name_upper.contains("SPARKLE") || name_upper.contains("TROPHY") {
        "Admiration".to_string()
    } else if name_upper.contains("THUMB") || name_upper.contains("CHECK") || name_upper.contains("OK") {
        "Approval".to_string()
    } else if pos > neg && pos > neu {
        "Joy".to_string()
    } else if neg > pos {
        "Disappointment".to_string()
    } else {
        "Neutral".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_embedded_dataset() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        assert!(dataset.len() > 750);

        let tear = dataset.get_by_char("😂").unwrap();
        assert_eq!(tear.name, "FACE WITH TEARS OF JOY");
        assert!(tear.score() > 0.0);

        let pleading = dataset.get_by_char("🥺").unwrap();
        assert_eq!(pleading.name, "PLEADING FACE");
        assert_eq!(pleading.primary_emotion, "Remorse");

        let adm = dataset.get_goemotion_category("Admiration").unwrap();
        assert_eq!(adm.polarity, "Positive");
        assert!(adm.mapped_emojis.contains(&"👑".to_string()));
    }
}
