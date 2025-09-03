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
            Self::parse_embedded_frames(&data[pos..], version_major)
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

    /// Parse embedded sub-frames from CHAP frame data
    fn parse_embedded_frames(frame_data: &[u8], version_major: u8) -> Vec<Id3v2Frame> {
        let mut embedded_frames = Vec::new();
        let mut pos = 0;

        while pos + 10 <= frame_data.len() {
            // Try to parse a sub-frame
            let frame_id = String::from_utf8_lossy(&frame_data[pos..pos + 4]).to_string();

            // Check if we've reached padding or end of data
            if frame_id.starts_with('\0') || !frame_id.chars().all(|c| c.is_ascii_alphanumeric()) {
                break;
            }

            // Validate frame ID for the given version
            if !crate::id3v2_tools::is_valid_frame_for_version(&frame_id, version_major) {
                break;
            }

            // Parse frame size based on ID3v2 version
            let frame_size = if version_major == 4 {
                // ID3v2.4 uses synchsafe integers
                crate::id3v2_tools::decode_synchsafe_int(&frame_data[pos + 4..pos + 8])
            } else {
                // ID3v2.3 uses big-endian integers
                u32::from_be_bytes([frame_data[pos + 4], frame_data[pos + 5], frame_data[pos + 6], frame_data[pos + 7]])
            };

            let frame_flags = u16::from_be_bytes([frame_data[pos + 8], frame_data[pos + 9]]);

            // Ensure we have enough data for the complete frame
            if pos + 10 + frame_size as usize > frame_data.len() {
                break;
            }

            // Extract frame data
            let data = frame_data[pos + 10..pos + 10 + frame_size as usize].to_vec();

            // Create the embedded frame
            let mut embedded_frame = Id3v2Frame::new(frame_id, frame_size, frame_flags, data);

            // Parse the embedded frame content for rich display
            if let Err(_e) = embedded_frame.parse_content(version_major) {
                // If parsing fails, we still keep the frame with raw data
            }

            embedded_frames.push(embedded_frame);

            // Move to next frame
            pos += 10 + frame_size as usize;
        }

        embedded_frames
    }
}
