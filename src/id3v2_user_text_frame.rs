/// User-Defined Text Information Frame (TXXX)
///
/// Structure: Text encoding + Description + Value

use crate::id3v2_text_encoding::{TextEncoding, split_terminated_text};

#[derive(Debug, Clone)]
pub struct UserTextFrame {
    pub encoding: TextEncoding,
    pub description: String,
    pub value: String,
}

impl UserTextFrame {
    /// Parse a TXXX frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.is_empty() {
            return Err("User text frame data is empty".to_string());
        }

        let encoding = TextEncoding::from_byte(data[0])?;
        if data.len() < 2 {
            return Err("User text frame data too short".to_string());
        }

        let text_data = &data[1..];
        let (description, value) = split_terminated_text(text_data, encoding)?;

        Ok(UserTextFrame {
            encoding,
            description,
            value,
        })
    }
}
