pub mod analyzer;
pub mod dataset;
pub mod formatter;

pub use analyzer::{
    AnalysisResult, Analyzer, EmojiBurst, EmojiCombo, EmojiStat, EmotionProfile, EmotionStat,
    FileReport, MultiFileAnalysisResult, PositionBias, SentimentProgression, SentimentSegment,
    SplitMode, UnicodeBlockStat,
};
pub use dataset::{EmojiData, EmojiDataset, GoEmotionCategory};
pub use formatter::{render_multi_output, render_output, OutputFormat};
