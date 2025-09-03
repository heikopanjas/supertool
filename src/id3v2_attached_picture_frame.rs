/// Attached Picture Frame (APIC)
///
/// Structure: Text encoding + MIME type + Picture type + Description + Picture data
use crate::id3v2_text_encoding::{TextEncoding, decode_iso88591_string, decode_text_with_encoding_simple, get_terminator_length, is_null_terminator};

#[derive(Debug, Clone)]
pub struct AttachedPictureFrame {
    pub encoding: TextEncoding,
    pub mime_type: String,
    pub picture_type: u8,
    pub description: String,
    pub picture_data: Vec<u8>,
}

impl AttachedPictureFrame {
    /// Parse an APIC frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 2 {
            return Err("Picture frame data too short".to_string());
        }

        let encoding = TextEncoding::from_byte(data[0])?;
        let mut pos = 1;

        // MIME type (null-terminated, ISO-8859-1)
        let mime_start = pos;
        while pos < data.len() && data[pos] != 0 {
            pos += 1;
        }
        if pos >= data.len() {
            return Err("Picture frame MIME type not null-terminated".to_string());
        }
        let mime_type = decode_iso88591_string(&data[mime_start..pos]);
        pos += 1; // Skip null terminator

        // Picture type (1 byte)
        if pos >= data.len() {
            return Err("Picture frame missing picture type".to_string());
        }
        let picture_type = data[pos];
        pos += 1;

        // Description (null-terminated, according to encoding)
        let desc_start = pos;
        let terminator_len = get_terminator_length(encoding);

        // Find description terminator
        while pos + terminator_len <= data.len() {
            if is_null_terminator(&data[pos..pos + terminator_len], encoding) {
                break;
            }
            pos += 1;
        }
        if pos + terminator_len > data.len() {
            return Err("Picture frame description not properly terminated".to_string());
        }

        let description = decode_text_with_encoding_simple(&data[desc_start..pos], encoding)?;
        pos += terminator_len; // Skip terminator

        // Picture data (rest of the frame)
        let picture_data = data[pos..].to_vec();

        Ok(AttachedPictureFrame { encoding, mime_type, picture_type, description, picture_data })
    }

    /// Get picture type description
    pub fn picture_type_description(&self) -> &'static str {
        match self.picture_type {
            | 0x00 => "Other",
            | 0x01 => "32x32 pixels 'file icon' (PNG only)",
            | 0x02 => "Other file icon",
            | 0x03 => "Cover (front)",
            | 0x04 => "Cover (back)",
            | 0x05 => "Leaflet page",
            | 0x06 => "Media (e.g. label side of CD)",
            | 0x07 => "Lead artist/lead performer/soloist",
            | 0x08 => "Artist/performer",
            | 0x09 => "Conductor",
            | 0x0A => "Band/Orchestra",
            | 0x0B => "Composer",
            | 0x0C => "Lyricist/text writer",
            | 0x0D => "Recording Location",
            | 0x0E => "During recording",
            | 0x0F => "During performance",
            | 0x10 => "Movie/video screen capture",
            | 0x11 => "A bright coloured fish",
            | 0x12 => "Illustration",
            | 0x13 => "Band/artist logotype",
            | 0x14 => "Publisher/Studio logotype",
            | _ => "Unknown",
        }
    }
}
