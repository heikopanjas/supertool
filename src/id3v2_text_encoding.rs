/// ID3v2 text encoding support and utilities
///
/// This module provides the `TextEncoding` enum and helper functions for decoding
/// text in various encodings used by ID3v2 frames.
use std::fmt;

/// Text encoding types used in ID3v2 frames
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextEncoding {
    /// ISO-8859-1 (Latin-1)
    Iso88591 = 0,
    /// UTF-16 with BOM
    Utf16Bom = 1,
    /// UTF-16BE (big-endian, no BOM) - ID3v2.4 only
    Utf16Be = 2,
    /// UTF-8 - ID3v2.4 only
    Utf8 = 3,
}

impl TextEncoding {
    /// Create TextEncoding from byte value
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            | 0 => Ok(TextEncoding::Iso88591),
            | 1 => Ok(TextEncoding::Utf16Bom),
            | 2 => Ok(TextEncoding::Utf16Be),
            | 3 => Ok(TextEncoding::Utf8),
            | _ => Err(format!("Unknown text encoding: {}", byte)),
        }
    }

    /// Check if encoding is valid for ID3v2 version
    pub fn is_valid_for_version(&self, version_major: u8) -> bool {
        match self {
            | TextEncoding::Iso88591 | TextEncoding::Utf16Bom => true,
            | TextEncoding::Utf16Be | TextEncoding::Utf8 => version_major >= 4,
        }
    }
}

impl fmt::Display for TextEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            | TextEncoding::Iso88591 => "ISO-8859-1",
            | TextEncoding::Utf16Bom => "UTF-16 with BOM",
            | TextEncoding::Utf16Be => "UTF-16BE",
            | TextEncoding::Utf8 => "UTF-8",
        };
        write!(f, "{}", name)
    }
}

// Helper functions for text decoding

/// Decode text with specified encoding, handling multiple null-separated strings
pub fn decode_text_with_encoding(data: &[u8], encoding: TextEncoding) -> Result<(String, Vec<String>), String> {
    let mut strings = Vec::new();
    let mut pos = 0;
    let terminator_len = get_terminator_length(encoding);

    while pos < data.len() {
        // Find next terminator
        let start = pos;
        while pos + terminator_len <= data.len() {
            if is_null_terminator(&data[pos..pos + terminator_len], encoding) {
                break;
            }
            pos += 1;
        }

        if start < pos {
            let text = decode_text_with_encoding_simple(&data[start..pos], encoding)?;
            if !text.is_empty() {
                strings.push(text);
            }
        }

        // Skip terminator
        if pos + terminator_len <= data.len() {
            pos += terminator_len;
        } else {
            // No terminator at end, include remaining data
            if pos < data.len() {
                let text = decode_text_with_encoding_simple(&data[pos..], encoding)?;
                if !text.is_empty() {
                    strings.push(text);
                }
            }
            break;
        }
    }

    let primary_text = if strings.is_empty() {
        String::new()
    } else {
        strings[0].clone()
    };

    Ok((primary_text, strings))
}

/// Decode single text string with specified encoding
pub fn decode_text_with_encoding_simple(data: &[u8], encoding: TextEncoding) -> Result<String, String> {
    match encoding {
        | TextEncoding::Iso88591 => Ok(decode_iso88591_string(data)),
        | TextEncoding::Utf8 => Ok(String::from_utf8_lossy(data).to_string()),
        | TextEncoding::Utf16Bom | TextEncoding::Utf16Be => decode_utf16_string(data, encoding),
    }
}

/// Split text data into two parts at first null terminator
pub fn split_terminated_text(data: &[u8], encoding: TextEncoding) -> Result<(String, String), String> {
    let (first_bytes, second_bytes) = find_text_terminator(data, encoding)?;
    let first = decode_text_with_encoding_simple(first_bytes, encoding)?;
    let second = decode_text_with_encoding_simple(second_bytes, encoding)?;
    Ok((first, second))
}

/// Find first null terminator and split data
pub fn find_text_terminator(data: &[u8], encoding: TextEncoding) -> Result<(&[u8], &[u8]), String> {
    let terminator_len = get_terminator_length(encoding);
    let mut pos = 0;

    while pos + terminator_len <= data.len() {
        if is_null_terminator(&data[pos..pos + terminator_len], encoding) {
            return Ok((&data[0..pos], &data[pos + terminator_len..]));
        }
        pos += 1;
    }

    // No terminator found, treat all data as first part
    Ok((data, &[]))
}

/// Get null terminator length for encoding
pub fn get_terminator_length(encoding: TextEncoding) -> usize {
    match encoding {
        | TextEncoding::Iso88591 | TextEncoding::Utf8 => 1,
        | TextEncoding::Utf16Bom | TextEncoding::Utf16Be => 2,
    }
}

/// Check if bytes represent null terminator for encoding
pub fn is_null_terminator(bytes: &[u8], encoding: TextEncoding) -> bool {
    match encoding {
        | TextEncoding::Iso88591 | TextEncoding::Utf8 => bytes.len() >= 1 && bytes[0] == 0,
        | TextEncoding::Utf16Bom | TextEncoding::Utf16Be => bytes.len() >= 2 && bytes[0] == 0 && bytes[1] == 0,
    }
}

/// Decode ISO-8859-1 string
pub fn decode_iso88591_string(data: &[u8]) -> String {
    data.iter().map(|&b| b as char).collect()
}

/// Decode UTF-16 string
pub fn decode_utf16_string(data: &[u8], encoding: TextEncoding) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let (start_pos, is_little_endian) = match encoding {
        | TextEncoding::Utf16Bom => {
            if data.len() >= 2 {
                if data[0] == 0xFF && data[1] == 0xFE {
                    (2, true) // Little endian BOM
                } else if data[0] == 0xFE && data[1] == 0xFF {
                    (2, false) // Big endian BOM
                } else {
                    (0, false) // No BOM, assume big endian
                }
            } else {
                (0, false)
            }
        }
        | TextEncoding::Utf16Be => (0, false), // Always big endian
        | _ => return Err("Invalid UTF-16 encoding".to_string()),
    };

    let utf16_data = &data[start_pos..];
    if utf16_data.len() % 2 != 0 {
        return Err("UTF-16 data length must be even".to_string());
    }

    let mut utf16_chars = Vec::new();
    for i in (0..utf16_data.len()).step_by(2) {
        let code_unit = if is_little_endian {
            u16::from_le_bytes([utf16_data[i], utf16_data[i + 1]])
        } else {
            u16::from_be_bytes([utf16_data[i], utf16_data[i + 1]])
        };
        utf16_chars.push(code_unit);
    }

    String::from_utf16(&utf16_chars).map_err(|_| "Invalid UTF-16 sequence".to_string())
}
