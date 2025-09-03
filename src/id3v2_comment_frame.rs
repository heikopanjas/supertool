/// Comment Frame (COMM, USLT)
///
/// Structure: Text encoding + Language + Short description + Full text
use crate::id3v2_text_encoding::{TextEncoding, split_terminated_text};

#[derive(Debug, Clone)]
pub struct CommentFrame {
    pub encoding: TextEncoding,
    pub language: String,
    pub description: String,
    pub text: String,
}

impl CommentFrame {
    /// Parse a COMM or USLT frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 5 {
            return Err("Comment frame data too short".to_string());
        }

        let encoding = TextEncoding::from_byte(data[0])?;

        // Language is always 3 bytes (ISO-639-2)
        let language_bytes = &data[1..4];
        let language = String::from_utf8_lossy(language_bytes).to_string();

        let text_data = &data[4..];
        let (description, text) = split_terminated_text(text_data, encoding)?;

        Ok(CommentFrame { encoding, language, description, text })
    }
}
