/// Text Information Frame (T*** frames except TXXX)
///
/// Structure: Text encoding + Information
/// Examples: TIT2, TALB, TPE1, TPE2, TCON, TYER, etc.
use crate::id3v2_text_encoding::{TextEncoding, decode_text_with_encoding};

#[derive(Debug, Clone)]
pub struct TextFrame {
    pub encoding: TextEncoding,
    pub text: String,
    /// Multiple strings (null-separated in original data)
    pub strings: Vec<String>,
}

impl TextFrame {
    /// Parse a text frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Text frame data is empty".to_string());
        }

        let encoding = TextEncoding::from_byte(data[0])?;
        if data.len() < 2 {
            return Err("Text frame data too short".to_string());
        }

        let text_data = &data[1..];
        let (text, strings) = decode_text_with_encoding(text_data, encoding)?;

        Ok(TextFrame { encoding, text, strings })
    }

    /// Get the first (primary) text string
    pub fn primary_text(&self) -> &str {
        &self.text
    }
}
