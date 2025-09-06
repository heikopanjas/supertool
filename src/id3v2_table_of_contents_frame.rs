use crate::id3v2_frame::Id3v2Frame;
/// Table of Contents Frame (CTOC)
///
/// Structure: Element ID + TOC flags + Entry count + Child element IDs + Sub-frames
/// Part of ID3v2 Chapter Frame Addendum specification
use crate::id3v2_text_encoding::decode_iso88591_string;
use std::fmt;

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
            crate::id3v2_tools::parse_embedded_frames(&data[pos..], version_major)
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
}

impl fmt::Display for TableOfContentsFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Element ID: \"{}\"", self.element_id)?;
        writeln!(f, "Flags: Top-level: {}, Ordered: {}", self.top_level, self.ordered)?;
        writeln!(f, "Child elements ({}):", self.child_count())?;
        for (i, child_id) in self.child_element_ids.iter().enumerate() {
            writeln!(f, "  [{}] \"{}\"", i + 1, child_id)?;
        }
        if self.has_sub_frames() {
            writeln!(f, "Sub-frames: {} embedded frame(s)", self.sub_frames.len())?;
            for (i, sub_frame) in self.sub_frames.iter().enumerate() {
                // Display content with embedded frame formatting helper (same as CHAP)
                crate::id3v2_chapter_frame::display_embedded_frame_content(f, sub_frame)?;
                // Add newline between embedded frames but not after the last one
                if i < self.sub_frames.len() - 1 {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}
