use crate::id3v2_frame::Id3v2Frame;
/// Chapter Frame (CHAP)
///
/// Structure: Element ID + Start time + End time + Start offset + End offset + Sub-frames
/// Part of ID3v2 Chapter Frame Addendum specification
use crate::id3v2_text_encoding::decode_iso88591_string;

#[derive(Debug, Clone)]
pub struct ChapterFrame {
    /// Element ID (null-terminated)
    pub element_id: String,
    /// Start time in milliseconds
    pub start_time: u32,
    /// End time in milliseconds
    pub end_time: u32,
    /// Start byte offset (0xFFFFFFFF if not used)
    pub start_offset: u32,
    /// End byte offset (0xFFFFFFFF if not used)
    pub end_offset: u32,
    /// Embedded sub-frames (optional)
    pub sub_frames: Vec<Id3v2Frame>,
}

impl ChapterFrame {
    /// Parse a CHAP frame from raw data
    pub fn parse(data: &[u8], version_major: u8) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Chapter frame data is empty".to_string());
        }

        let mut pos = 0;

        // Element ID (null-terminated ISO-8859-1)
        let element_id_start = pos;
        while pos < data.len() && data[pos] != 0 {
            pos += 1;
        }
        if pos >= data.len() {
            return Err("Chapter frame element ID not null-terminated".to_string());
        }
        let element_id = decode_iso88591_string(&data[element_id_start..pos]);
        pos += 1; // Skip null terminator

        // Start time (4 bytes)
        if pos + 4 > data.len() {
            return Err("Chapter frame missing start time".to_string());
        }
        let start_time = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // End time (4 bytes)
        if pos + 4 > data.len() {
            return Err("Chapter frame missing end time".to_string());
        }
        let end_time = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // Start offset (4 bytes)
        if pos + 4 > data.len() {
            return Err("Chapter frame missing start offset".to_string());
        }
        let start_offset = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // End offset (4 bytes)
        if pos + 4 > data.len() {
            return Err("Chapter frame missing end offset".to_string());
        }
        let end_offset = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // Parse embedded sub-frames (rest of the data)
        let sub_frames = if pos < data.len() {
            let (_, frames) = crate::id3v2_tools::parse_embedded_frames(&data[pos..], 0, version_major);
            frames
        } else {
            Vec::new()
        };

        Ok(ChapterFrame { element_id, start_time, end_time, start_offset, end_offset, sub_frames })
    }

    /// Check if byte offsets are used (not 0xFFFFFFFF)
    pub fn has_byte_offsets(&self) -> bool {
        self.start_offset != 0xFFFFFFFF && self.end_offset != 0xFFFFFFFF
    }

    /// Get chapter duration in milliseconds
    pub fn duration(&self) -> u32 {
        if self.end_time >= self.start_time {
            self.end_time - self.start_time
        } else {
            0
        }
    }
}
