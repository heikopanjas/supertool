use crate::id3v2_frame::Id3v2Frame;
/// Chapter Frame (CHAP)
///
/// Structure: Element ID + Start time + End time + Start offset + End offset + Sub-frames
/// Part of ID3v2 Chapter Frame Addendum specification
use crate::id3v2_text_encoding::decode_iso88591_string;
use crate::id3v2_tools::get_frame_description;
use std::fmt;

/// Format milliseconds as hh:mm:ss.ms
fn format_timestamp(ms: u32) -> String {
    let total_seconds = ms / 1000;
    let milliseconds = ms % 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
}

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
            crate::id3v2_tools::parse_embedded_frames(&data[pos..], version_major)
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

impl fmt::Display for ChapterFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Element ID: \"{}\"", self.element_id)?;
        let start_formatted = format_timestamp(self.start_time);
        let end_formatted = format_timestamp(self.end_time);
        let duration_formatted = format_timestamp(self.duration());
        writeln!(f, "Time: {} - {} (duration: {})", start_formatted, end_formatted, duration_formatted)?;
        if self.has_byte_offsets() {
            writeln!(f, "Byte offsets: {} - {}", self.start_offset, self.end_offset)?;
        }
        if !self.sub_frames.is_empty() {
            writeln!(f, "Sub-frames: {} embedded frame(s)", self.sub_frames.len())?;
            writeln!(f)?; // Add newline before first embedded frame
            for (i, sub_frame) in self.sub_frames.iter().enumerate() {
                // Display content with embedded frame formatting helper
                display_embedded_frame_content(f, sub_frame)?;
                // Add newline between embedded frames but not after the last one
                if i < self.sub_frames.len() - 1 {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

/// Helper function to display embedded frame content with proper indentation matching top-level format
pub fn display_embedded_frame_content(f: &mut fmt::Formatter<'_>, frame: &Id3v2Frame) -> fmt::Result {
    // Use the new unified frame header display function
    let mut buffer = Vec::new();
    if let Err(_) = crate::id3v2_tools::display_frame_header(&mut buffer, frame, "        ") {
        // Fallback to basic display if header function fails
        writeln!(f, "        Frame: {} - Size: {} bytes", frame.id, frame.size)?;
    } else {
        // Convert buffer to string and write to formatter
        if let Ok(header_str) = String::from_utf8(buffer) {
            write!(f, "{}", header_str)?;
        }
    }

    // Format embedded frames like top-level frames but with embedded indentation
    writeln!(f, "        Frame: {} ({}) - Size: {} bytes", frame.id, get_frame_description(&frame.id), frame.size)?;

    if let Some(content) = &frame.content {
        // Add content with additional indentation (12 spaces total: 8 for embedded + 4 for content)
        let content_str = format!("{}", content);
        for line in content_str.lines() {
            if !line.is_empty() {
                writeln!(f, "            {}", line)?;
            } else {
                writeln!(f)?;
            }
        }
    } else {
        // Fallback for unparsed frames
        if let Some(text) = frame.get_text() {
            if !text.is_empty() {
                writeln!(f, "            Text: \"{}\"", text)?;
            }
        } else if let Some(url) = frame.get_url() {
            writeln!(f, "            URL: \"{}\"", url)?;
        }
    }
    Ok(())
}
