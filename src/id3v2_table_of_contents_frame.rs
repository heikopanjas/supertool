use crate::id3v2_frame::Id3v2Frame;
/// Table of Contents Frame (CTOC)
///
/// Structure: Element ID + TOC flags + Entry count + Child element IDs + Sub-frames
/// Part of ID3v2 Chapter Frame Addendum specification
use crate::id3v2_text_encoding::decode_iso88591_string;

#[derive(Debug, Clone)]
pub struct TableOfContentsFrame {
    /// Element ID (null-terminated)
    pub element_id: String,
    /// Top-level flag (true if this is a top-level TOC)
    pub top_level: bool,
    /// Ordered flag (true if child elements are ordered)
    pub ordered: bool,
    /// Child element IDs
    pub child_element_ids: Vec<String>,
    /// Embedded sub-frames (optional)
    pub sub_frames: Vec<Id3v2Frame>,
}

impl TableOfContentsFrame {
    /// Parse a CTOC frame from raw data
    pub fn parse(data: &[u8], version_major: u8) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Table of contents frame data is empty".to_string());
        }

        let mut pos = 0;

        // Element ID (null-terminated ISO-8859-1)
        let element_id_start = pos;
        while pos < data.len() && data[pos] != 0 {
            pos += 1;
        }
        if pos >= data.len() {
            return Err("TOC frame element ID not null-terminated".to_string());
        }
        let element_id = decode_iso88591_string(&data[element_id_start..pos]);
        pos += 1; // Skip null terminator

        // TOC flags (1 byte)
        if pos >= data.len() {
            return Err("TOC frame missing flags".to_string());
        }
        let flags = data[pos];
        pos += 1;

        let top_level = (flags & 0x02) != 0;
        let ordered = (flags & 0x01) != 0;

        // Entry count (1 byte)
        if pos >= data.len() {
            return Err("TOC frame missing entry count".to_string());
        }
        let entry_count = data[pos];
        pos += 1;

        // Child element IDs (null-terminated strings)
        let mut child_element_ids = Vec::new();
        for _ in 0..entry_count {
            let id_start = pos;
            while pos < data.len() && data[pos] != 0 {
                pos += 1;
            }
            if pos >= data.len() {
                return Err("TOC frame child element ID not null-terminated".to_string());
            }
            let child_id = decode_iso88591_string(&data[id_start..pos]);
            child_element_ids.push(child_id);
            pos += 1; // Skip null terminator
        }

        // Parse embedded sub-frames (rest of the data)
        let sub_frames = if pos < data.len() {
            Self::parse_embedded_frames(&data[pos..], version_major)
        } else {
            Vec::new()
        };

        Ok(TableOfContentsFrame { element_id, top_level, ordered, child_element_ids, sub_frames })
    }

    /// Get number of child elements
    pub fn child_count(&self) -> usize {
        self.child_element_ids.len()
    }

    /// Check if this TOC has sub-frames
    pub fn has_sub_frames(&self) -> bool {
        !self.sub_frames.is_empty()
    }

    /// Parse embedded sub-frames from CTOC frame data
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
