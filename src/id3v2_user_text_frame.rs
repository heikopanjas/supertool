/// User-Defined Text Information Frame (TXXX)
///
/// Structure: Text encoding + Description + Value
use crate::id3v2_text_encoding::{TextEncoding, split_terminated_text};
use std::fmt;

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

        Ok(UserTextFrame { encoding, description, value })
    }
}

impl fmt::Display for UserTextFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Encoding: {}", self.encoding)?;
        writeln!(f, "Description: \"{}\"", self.description)?;
        if self.value.len() > 100 {
            writeln!(f, "Value: \"{}...\"", self.value.chars().take(100).collect::<String>())?;
        } else {
            writeln!(f, "Value: \"{}\"", self.value)?;
        }
        Ok(())
    }
}
