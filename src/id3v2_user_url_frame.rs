/// User-Defined URL Link Frame (WXXX)
///
/// Structure: Text encoding + Description + URL
use crate::id3v2_text_encoding::{TextEncoding, decode_iso88591_string, decode_text_with_encoding_simple, find_text_terminator};
use std::fmt;

#[derive(Debug, Clone)]
pub struct UserUrlFrame {
    pub encoding: TextEncoding,
    pub description: String,
    pub url: String,
}

impl UserUrlFrame {
    /// Parse a WXXX frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.is_empty() {
            return Err("User URL frame data is empty".to_string());
        }

        let encoding = TextEncoding::from_byte(data[0])?;
        if data.len() < 2 {
            return Err("User URL frame data too short".to_string());
        }

        let text_data = &data[1..];

        // Find the null terminator for description
        let (description_bytes, url_bytes) = find_text_terminator(text_data, encoding)?;
        let description = decode_text_with_encoding_simple(description_bytes, encoding)?;

        // URL is always ISO-8859-1
        let url = decode_iso88591_string(url_bytes);

        Ok(UserUrlFrame { encoding, description, url })
    }
}

impl fmt::Display for UserUrlFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Encoding: {}", self.encoding)?;
        writeln!(f, "Description: \"{}\"", self.description)?;
        writeln!(f, "URL: \"{}\"", self.url)?;
        Ok(())
    }
}
