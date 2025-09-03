use crate::id3v2_attached_picture_frame::AttachedPictureFrame;
use crate::id3v2_chapter_frame::ChapterFrame;
use crate::id3v2_comment_frame::CommentFrame;
use crate::id3v2_table_of_contents_frame::TableOfContentsFrame;
use crate::id3v2_text_frame::TextFrame;
use crate::id3v2_tools::get_frame_description;
use crate::id3v2_unique_file_id_frame::UniqueFileIdFrame;
use crate::id3v2_url_frame::UrlFrame;
use crate::id3v2_user_text_frame::UserTextFrame;
use crate::id3v2_user_url_frame::UserUrlFrame;
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

/// Parsed content of an ID3v2 frame
#[derive(Debug, Clone)]
pub enum Id3v2FrameContent {
    /// Text information frame (T*** except TXXX)
    Text(TextFrame),
    /// URL link frame (W*** except WXXX)
    Url(UrlFrame),
    /// User-defined text frame (TXXX)
    UserText(UserTextFrame),
    /// User-defined URL frame (WXXX)
    UserUrl(UserUrlFrame),
    /// Comment frame (COMM, USLT)
    Comment(CommentFrame),
    /// Attached picture frame (APIC)
    Picture(AttachedPictureFrame),
    /// Unique file identifier (UFID)
    UniqueFileId(UniqueFileIdFrame),
    /// Chapter frame (CHAP)
    Chapter(ChapterFrame),
    /// Table of contents frame (CTOC)
    TableOfContents(TableOfContentsFrame),
    /// Raw binary data for unsupported/unknown frames
    Binary(Vec<u8>),
}

/// ID3v2 frame representation for all versions
#[derive(Debug, Clone)]
pub struct Id3v2Frame {
    /// Four-character frame identifier (e.g., "TIT2", "TPE1", "TALB")
    pub id: String,
    /// Size of the frame data (excluding header)
    pub size: u32,
    /// Frame flags (meaning varies by ID3v2 version)
    pub flags: u16,
    /// Raw frame data content
    pub data: Vec<u8>,
    /// Parsed frame content (if successfully parsed)
    pub content: Option<Id3v2FrameContent>,
    /// Embedded sub-frames (for CHAP and CTOC frames)
    pub embedded_frames: Option<Vec<Id3v2Frame>>,
}

impl Id3v2Frame {
    /// Create a new ID3v2 frame with raw data only
    pub fn new(id: String, size: u32, flags: u16, data: Vec<u8>) -> Self {
        Self { id, size, flags, data, content: None, embedded_frames: None }
    }

    /// Create a new ID3v2 frame with parsed content
    pub fn new_with_content(id: String, size: u32, flags: u16, data: Vec<u8>, content: Id3v2FrameContent) -> Self {
        Self { id, size, flags, data, content: Some(content), embedded_frames: None }
    }

    /// Create a new ID3v2 frame with embedded sub-frames (for CHAP/CTOC frames)
    pub fn new_with_embedded(id: String, size: u32, flags: u16, data: Vec<u8>, embedded_frames: Vec<Id3v2Frame>) -> Self {
        Self { id, size, flags, data, content: None, embedded_frames: Some(embedded_frames) }
    }

    /// Create a new ID3v2 frame with both content and embedded frames
    pub fn new_complete(id: String, size: u32, flags: u16, data: Vec<u8>, content: Option<Id3v2FrameContent>, embedded_frames: Option<Vec<Id3v2Frame>>) -> Self {
        Self { id, size, flags, data, content, embedded_frames }
    }

    /// Parse frame content based on frame ID
    pub fn parse_content(&mut self, version_major: u8) -> Result<(), String> {
        // Validate that this frame is valid for the given ID3v2 version
        if !crate::id3v2_tools::is_valid_frame_for_version(&self.id, version_major) {
            // Invalid frame for this version, store as binary data
            self.content = Some(Id3v2FrameContent::Binary(self.data.clone()));
            return Ok(());
        }

        let content = match self.id.as_str() {
            // Text information frames
            | id if id.starts_with('T') && id != "TXXX" => {
                let text_frame = TextFrame::parse(&self.data)?;
                // Validate text encoding for this ID3v2 version
                if !text_frame.encoding.is_valid_for_version(version_major) {
                    return Err(format!("Text encoding {:?} is not valid for ID3v2.{}", text_frame.encoding, version_major));
                }
                Id3v2FrameContent::Text(text_frame)
            }
            // URL link frames (no encoding to validate)
            | id if id.starts_with('W') && id != "WXXX" => Id3v2FrameContent::Url(UrlFrame::parse(&self.data)?),
            // User-defined frames
            | "TXXX" => {
                let user_text_frame = UserTextFrame::parse(&self.data)?;
                // Validate text encoding for this ID3v2 version
                if !user_text_frame.encoding.is_valid_for_version(version_major) {
                    return Err(format!("Text encoding {:?} is not valid for ID3v2.{}", user_text_frame.encoding, version_major));
                }
                Id3v2FrameContent::UserText(user_text_frame)
            }
            | "WXXX" => {
                let user_url_frame = UserUrlFrame::parse(&self.data)?;
                // Validate text encoding for this ID3v2 version
                if !user_url_frame.encoding.is_valid_for_version(version_major) {
                    return Err(format!("Text encoding {:?} is not valid for ID3v2.{}", user_url_frame.encoding, version_major));
                }
                Id3v2FrameContent::UserUrl(user_url_frame)
            }
            // Comment frames
            | "COMM" | "USLT" => {
                let comment_frame = CommentFrame::parse(&self.data)?;
                // Validate text encoding for this ID3v2 version
                if !comment_frame.encoding.is_valid_for_version(version_major) {
                    return Err(format!("Text encoding {:?} is not valid for ID3v2.{}", comment_frame.encoding, version_major));
                }
                Id3v2FrameContent::Comment(comment_frame)
            }
            // Attached picture
            | "APIC" => {
                let picture_frame = AttachedPictureFrame::parse(&self.data)?;
                // Validate text encoding for this ID3v2 version
                if !picture_frame.encoding.is_valid_for_version(version_major) {
                    return Err(format!("Text encoding {:?} is not valid for ID3v2.{}", picture_frame.encoding, version_major));
                }
                Id3v2FrameContent::Picture(picture_frame)
            }
            // Unique file identifier (no encoding)
            | "UFID" => Id3v2FrameContent::UniqueFileId(UniqueFileIdFrame::parse(&self.data)?),
            // Chapter frames (may contain sub-frames with their own validation)
            | "CHAP" => Id3v2FrameContent::Chapter(ChapterFrame::parse(&self.data, version_major)?),
            | "CTOC" => Id3v2FrameContent::TableOfContents(TableOfContentsFrame::parse(&self.data, version_major)?),
            // Other frames remain as binary data
            | _ => Id3v2FrameContent::Binary(self.data.clone()),
        };

        self.content = Some(content);
        Ok(())
    }

    /// Get the frame ID as a printable string
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the frame data size
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Get the frame flags
    pub fn flags(&self) -> u16 {
        self.flags
    }

    /// Get the frame data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Check if the frame ID is valid (printable ASCII alphanumeric)
    pub fn is_valid_id(&self) -> bool {
        self.id.len() == 4 && self.id.chars().all(|c| c.is_ascii_alphanumeric())
    }

    /// Get the total frame size including header (10 bytes for header + data size)
    pub fn total_size(&self) -> u32 {
        10 + self.size
    }

    /// Check if this frame type supports embedded sub-frames
    pub fn supports_embedded_frames(&self) -> bool {
        matches!(self.id.as_str(), "CHAP" | "CTOC")
    }

    /// Get embedded sub-frames (if any)
    pub fn embedded_frames(&self) -> Option<&Vec<Id3v2Frame>> {
        self.embedded_frames.as_ref()
    }

    /// Check if this frame has embedded sub-frames
    pub fn has_embedded_frames(&self) -> bool {
        self.embedded_frames.is_some() && !self.embedded_frames.as_ref().unwrap().is_empty()
    }

    /// Get text content if this is a text frame
    pub fn get_text(&self) -> Option<&str> {
        match &self.content {
            | Some(Id3v2FrameContent::Text(text_frame)) => Some(text_frame.primary_text()),
            | Some(Id3v2FrameContent::UserText(user_text_frame)) => Some(&user_text_frame.value),
            | Some(Id3v2FrameContent::Comment(comment_frame)) => Some(&comment_frame.text),
            | _ => None,
        }
    }

    /// Get URL if this is a URL frame
    pub fn get_url(&self) -> Option<&str> {
        match &self.content {
            | Some(Id3v2FrameContent::Url(url_frame)) => Some(&url_frame.url),
            | Some(Id3v2FrameContent::UserUrl(user_url_frame)) => Some(&user_url_frame.url),
            | _ => None,
        }
    }

    /// Check if frame content was successfully parsed
    pub fn is_parsed(&self) -> bool {
        self.content.is_some()
    }
}

impl fmt::Display for Id3v2Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame: {} ({})", self.id, get_frame_description(&self.id))?;
        write!(f, " - Size: {} bytes", self.size)?;

        if self.flags != 0 {
            write!(f, " - Flags: 0x{:04X}", self.flags)?;
        }

        // Show detailed parsed content based on frame type
        if let Some(content) = &self.content {
            match content {
                | Id3v2FrameContent::Text(text_frame) => {
                    writeln!(f)?;
                    write!(f, "    Encoding: {}", text_frame.encoding)?;
                    if text_frame.strings.len() > 1 {
                        writeln!(f)?;
                        write!(f, "    Values ({} strings):", text_frame.strings.len())?;
                        for (i, string) in text_frame.strings.iter().enumerate() {
                            writeln!(f)?;
                            if string.len() > 80 {
                                write!(f, "      [{}] \"{}...\"", i + 1, string.chars().take(80).collect::<String>())?;
                            } else {
                                write!(f, "      [{}] \"{}\"", i + 1, string)?;
                            }
                        }
                    } else if !text_frame.text.is_empty() {
                        writeln!(f)?;
                        if text_frame.text.len() > 100 {
                            write!(f, "    Value: \"{}...\"", text_frame.text.chars().take(100).collect::<String>())?;
                        } else {
                            write!(f, "    Value: \"{}\"", text_frame.text)?;
                        }
                    }
                }
                | Id3v2FrameContent::UserText(user_text_frame) => {
                    writeln!(f)?;
                    write!(f, "    Encoding: {}", user_text_frame.encoding)?;
                    writeln!(f)?;
                    write!(f, "    Description: \"{}\"", user_text_frame.description)?;
                    writeln!(f)?;
                    if user_text_frame.value.len() > 100 {
                        write!(f, "    Value: \"{}...\"", user_text_frame.value.chars().take(100).collect::<String>())?;
                    } else {
                        write!(f, "    Value: \"{}\"", user_text_frame.value)?;
                    }
                }
                | Id3v2FrameContent::Url(url_frame) => {
                    writeln!(f)?;
                    write!(f, "    URL: \"{}\"", url_frame.url)?;
                }
                | Id3v2FrameContent::UserUrl(user_url_frame) => {
                    writeln!(f)?;
                    write!(f, "    Encoding: {}", user_url_frame.encoding)?;
                    writeln!(f)?;
                    write!(f, "    Description: \"{}\"", user_url_frame.description)?;
                    writeln!(f)?;
                    write!(f, "    URL: \"{}\"", user_url_frame.url)?;
                }
                | Id3v2FrameContent::Comment(comment_frame) => {
                    writeln!(f)?;
                    write!(f, "    Encoding: {}", comment_frame.encoding)?;
                    writeln!(f)?;
                    write!(f, "    Language: \"{}\"", comment_frame.language)?;
                    if !comment_frame.description.is_empty() {
                        writeln!(f)?;
                        write!(f, "    Description: \"{}\"", comment_frame.description)?;
                    }
                    writeln!(f)?;
                    if comment_frame.text.len() > 100 {
                        write!(f, "    Text: \"{}...\"", comment_frame.text.chars().take(100).collect::<String>())?;
                    } else {
                        write!(f, "    Text: \"{}\"", comment_frame.text)?;
                    }
                }
                | Id3v2FrameContent::Picture(picture_frame) => {
                    writeln!(f)?;
                    write!(f, "    Encoding: {}", picture_frame.encoding)?;
                    writeln!(f)?;
                    write!(f, "    MIME type: {}", picture_frame.mime_type)?;
                    writeln!(f)?;
                    write!(f, "    Picture type: {} ({})", picture_frame.picture_type, picture_frame.picture_type_description())?;
                    if !picture_frame.description.is_empty() {
                        writeln!(f)?;
                        write!(f, "    Description: \"{}\"", picture_frame.description)?;
                    }
                    writeln!(f)?;
                    write!(f, "    Data size: {} bytes", picture_frame.picture_data.len())?;
                }
                | Id3v2FrameContent::UniqueFileId(ufid_frame) => {
                    writeln!(f)?;
                    write!(f, "    Owner: \"{}\"", ufid_frame.owner_identifier)?;
                    writeln!(f)?;
                    write!(f, "    Identifier: {} bytes", ufid_frame.identifier.len())?;
                }
                | Id3v2FrameContent::Chapter(chapter_frame) => {
                    writeln!(f)?;
                    write!(f, "    Element ID: \"{}\"", chapter_frame.element_id)?;
                    writeln!(f)?;
                    let start_formatted = format_timestamp(chapter_frame.start_time);
                    let end_formatted = format_timestamp(chapter_frame.end_time);
                    let duration_formatted = format_timestamp(chapter_frame.duration());
                    write!(f, "    Time: {} - {} (duration: {})", start_formatted, end_formatted, duration_formatted)?;
                    if chapter_frame.has_byte_offsets() {
                        writeln!(f)?;
                        write!(f, "    Byte offsets: {} - {}", chapter_frame.start_offset, chapter_frame.end_offset)?;
                    }
                    if !chapter_frame.sub_frames.is_empty() {
                        writeln!(f)?;
                        write!(f, "    Sub-frames: {} embedded frame(s)", chapter_frame.sub_frames.len())?;
                        for (i, sub_frame) in chapter_frame.sub_frames.iter().enumerate() {
                            writeln!(f)?;
                            write!(f, "      [{}] {} - {}", i + 1, sub_frame.id, get_frame_description(&sub_frame.id))?;

                            // Show rich details for embedded frames
                            if let Some(content) = &sub_frame.content {
                                match content {
                                    | Id3v2FrameContent::Text(text_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", text_frame.encoding)?;
                                        if text_frame.strings.len() > 1 {
                                            writeln!(f)?;
                                            write!(f, "          Values ({} strings):", text_frame.strings.len())?;
                                            for (i, string) in text_frame.strings.iter().enumerate() {
                                                writeln!(f)?;
                                                if string.len() > 60 {
                                                    write!(f, "            [{}] \"{}...\"", i + 1, string.chars().take(60).collect::<String>())?;
                                                } else {
                                                    write!(f, "            [{}] \"{}\"", i + 1, string)?;
                                                }
                                            }
                                        } else if !text_frame.text.is_empty() {
                                            writeln!(f)?;
                                            let display_text = if text_frame.text.len() > 60 {
                                                format!("{}...", text_frame.text.chars().take(60).collect::<String>())
                                            } else {
                                                text_frame.text.clone()
                                            };
                                            write!(f, "          Value: \"{}\"", display_text)?;
                                        }
                                    }
                                    | Id3v2FrameContent::UserText(user_text_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", user_text_frame.encoding)?;
                                        writeln!(f)?;
                                        write!(f, "          Description: \"{}\"", user_text_frame.description)?;
                                        writeln!(f)?;
                                        let display_text = if user_text_frame.value.len() > 60 {
                                            format!("{}...", user_text_frame.value.chars().take(60).collect::<String>())
                                        } else {
                                            user_text_frame.value.clone()
                                        };
                                        write!(f, "          Value: \"{}\"", display_text)?;
                                    }
                                    | Id3v2FrameContent::Url(url_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          URL: \"{}\"", url_frame.url)?;
                                    }
                                    | Id3v2FrameContent::UserUrl(user_url_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", user_url_frame.encoding)?;
                                        writeln!(f)?;
                                        write!(f, "          Description: \"{}\"", user_url_frame.description)?;
                                        writeln!(f)?;
                                        write!(f, "          URL: \"{}\"", user_url_frame.url)?;
                                    }
                                    | Id3v2FrameContent::Comment(comment_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", comment_frame.encoding)?;
                                        writeln!(f)?;
                                        write!(f, "          Language: \"{}\"", comment_frame.language)?;
                                        if !comment_frame.description.is_empty() {
                                            writeln!(f)?;
                                            write!(f, "          Description: \"{}\"", comment_frame.description)?;
                                        }
                                        writeln!(f)?;
                                        let display_text = if comment_frame.text.len() > 60 {
                                            format!("{}...", comment_frame.text.chars().take(60).collect::<String>())
                                        } else {
                                            comment_frame.text.clone()
                                        };
                                        write!(f, "          Text: \"{}\"", display_text)?;
                                    }
                                    | Id3v2FrameContent::Picture(picture_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          MIME type: {}", picture_frame.mime_type)?;
                                        writeln!(f)?;
                                        write!(f, "          Picture type: {} ({})", picture_frame.picture_type, picture_frame.picture_type_description())?;
                                        if !picture_frame.description.is_empty() {
                                            writeln!(f)?;
                                            write!(f, "          Description: \"{}\"", picture_frame.description)?;
                                        }
                                        writeln!(f)?;
                                        write!(f, "          Data size: {} bytes", picture_frame.picture_data.len())?;
                                    }
                                    | Id3v2FrameContent::UniqueFileId(ufid_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Owner: \"{}\"", ufid_frame.owner_identifier)?;
                                        writeln!(f)?;
                                        write!(f, "          Identifier: {} bytes", ufid_frame.identifier.len())?;
                                    }
                                    | Id3v2FrameContent::Binary(_) => {
                                        writeln!(f)?;
                                        write!(f, "          Binary data: {} bytes", sub_frame.size)?;
                                    }
                                    | _ => {
                                        // For other frame types, show basic text/URL if available
                                        if let Some(text) = sub_frame.get_text() {
                                            if !text.is_empty() {
                                                writeln!(f)?;
                                                let display_text = if text.len() > 60 {
                                                    format!("{}...", text.chars().take(60).collect::<String>())
                                                } else {
                                                    text.to_string()
                                                };
                                                write!(f, "          Text: \"{}\"", display_text)?;
                                            }
                                        } else if let Some(url) = sub_frame.get_url() {
                                            writeln!(f)?;
                                            write!(f, "          URL: \"{}\"", url)?;
                                        }
                                    }
                                }
                            } else {
                                // Fallback for unparsed frames
                                if let Some(text) = sub_frame.get_text() {
                                    if !text.is_empty() {
                                        writeln!(f)?;
                                        let display_text = if text.len() > 60 {
                                            format!("{}...", text.chars().take(60).collect::<String>())
                                        } else {
                                            text.to_string()
                                        };
                                        write!(f, "          Text: \"{}\"", display_text)?;
                                    }
                                } else if let Some(url) = sub_frame.get_url() {
                                    writeln!(f)?;
                                    write!(f, "          URL: \"{}\"", url)?;
                                }
                            }
                        }
                    }
                }
                | Id3v2FrameContent::TableOfContents(toc_frame) => {
                    writeln!(f)?;
                    write!(f, "    Element ID: \"{}\"", toc_frame.element_id)?;
                    writeln!(f)?;
                    write!(f, "    Flags: Top-level: {}, Ordered: {}", toc_frame.top_level, toc_frame.ordered)?;
                    writeln!(f)?;
                    write!(f, "    Child elements ({}):", toc_frame.child_count())?;
                    for (i, child_id) in toc_frame.child_element_ids.iter().enumerate() {
                        writeln!(f)?;
                        write!(f, "      [{}] \"{}\"", i + 1, child_id)?;
                    }
                    if toc_frame.has_sub_frames() {
                        writeln!(f)?;
                        write!(f, "    Sub-frames: {} embedded frame(s)", toc_frame.sub_frames.len())?;
                        for (i, sub_frame) in toc_frame.sub_frames.iter().enumerate() {
                            writeln!(f)?;
                            write!(f, "      [{}] {} - {}", i + 1, sub_frame.id, get_frame_description(&sub_frame.id))?;

                            // Show rich details for embedded frames (same logic as CHAP frames)
                            if let Some(content) = &sub_frame.content {
                                match content {
                                    | Id3v2FrameContent::Text(text_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", text_frame.encoding)?;
                                        if text_frame.strings.len() > 1 {
                                            writeln!(f)?;
                                            write!(f, "          Values ({} strings):", text_frame.strings.len())?;
                                            for (i, string) in text_frame.strings.iter().enumerate() {
                                                writeln!(f)?;
                                                if string.len() > 60 {
                                                    write!(f, "            [{}] \"{}...\"", i + 1, string.chars().take(60).collect::<String>())?;
                                                } else {
                                                    write!(f, "            [{}] \"{}\"", i + 1, string)?;
                                                }
                                            }
                                        } else if !text_frame.text.is_empty() {
                                            writeln!(f)?;
                                            let display_text = if text_frame.text.len() > 60 {
                                                format!("{}...", text_frame.text.chars().take(60).collect::<String>())
                                            } else {
                                                text_frame.text.clone()
                                            };
                                            write!(f, "          Value: \"{}\"", display_text)?;
                                        }
                                    }
                                    | Id3v2FrameContent::UserText(user_text_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", user_text_frame.encoding)?;
                                        writeln!(f)?;
                                        write!(f, "          Description: \"{}\"", user_text_frame.description)?;
                                        writeln!(f)?;
                                        let display_text = if user_text_frame.value.len() > 60 {
                                            format!("{}...", user_text_frame.value.chars().take(60).collect::<String>())
                                        } else {
                                            user_text_frame.value.clone()
                                        };
                                        write!(f, "          Value: \"{}\"", display_text)?;
                                    }
                                    | Id3v2FrameContent::Url(url_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          URL: \"{}\"", url_frame.url)?;
                                    }
                                    | Id3v2FrameContent::UserUrl(user_url_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", user_url_frame.encoding)?;
                                        writeln!(f)?;
                                        write!(f, "          Description: \"{}\"", user_url_frame.description)?;
                                        writeln!(f)?;
                                        write!(f, "          URL: \"{}\"", user_url_frame.url)?;
                                    }
                                    | Id3v2FrameContent::Comment(comment_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Encoding: {}", comment_frame.encoding)?;
                                        writeln!(f)?;
                                        write!(f, "          Language: \"{}\"", comment_frame.language)?;
                                        if !comment_frame.description.is_empty() {
                                            writeln!(f)?;
                                            write!(f, "          Description: \"{}\"", comment_frame.description)?;
                                        }
                                        writeln!(f)?;
                                        let display_text = if comment_frame.text.len() > 60 {
                                            format!("{}...", comment_frame.text.chars().take(60).collect::<String>())
                                        } else {
                                            comment_frame.text.clone()
                                        };
                                        write!(f, "          Text: \"{}\"", display_text)?;
                                    }
                                    | Id3v2FrameContent::Picture(picture_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          MIME type: {}", picture_frame.mime_type)?;
                                        writeln!(f)?;
                                        write!(f, "          Picture type: {} ({})", picture_frame.picture_type, picture_frame.picture_type_description())?;
                                        if !picture_frame.description.is_empty() {
                                            writeln!(f)?;
                                            write!(f, "          Description: \"{}\"", picture_frame.description)?;
                                        }
                                        writeln!(f)?;
                                        write!(f, "          Data size: {} bytes", picture_frame.picture_data.len())?;
                                    }
                                    | Id3v2FrameContent::UniqueFileId(ufid_frame) => {
                                        writeln!(f)?;
                                        write!(f, "          Owner: \"{}\"", ufid_frame.owner_identifier)?;
                                        writeln!(f)?;
                                        write!(f, "          Identifier: {} bytes", ufid_frame.identifier.len())?;
                                    }
                                    | Id3v2FrameContent::Binary(_) => {
                                        writeln!(f)?;
                                        write!(f, "          Binary data: {} bytes", sub_frame.size)?;
                                    }
                                    | _ => {
                                        // For other frame types, show basic text/URL if available
                                        if let Some(text) = sub_frame.get_text() {
                                            if !text.is_empty() {
                                                writeln!(f)?;
                                                let display_text = if text.len() > 60 {
                                                    format!("{}...", text.chars().take(60).collect::<String>())
                                                } else {
                                                    text.to_string()
                                                };
                                                write!(f, "          Text: \"{}\"", display_text)?;
                                            }
                                        } else if let Some(url) = sub_frame.get_url() {
                                            writeln!(f)?;
                                            write!(f, "          URL: \"{}\"", url)?;
                                        }
                                    }
                                }
                            } else {
                                // Fallback for unparsed frames
                                if let Some(text) = sub_frame.get_text() {
                                    if !text.is_empty() {
                                        writeln!(f)?;
                                        let display_text = if text.len() > 60 {
                                            format!("{}...", text.chars().take(60).collect::<String>())
                                        } else {
                                            text.to_string()
                                        };
                                        write!(f, "          Text: \"{}\"", display_text)?;
                                    }
                                } else if let Some(url) = sub_frame.get_url() {
                                    writeln!(f)?;
                                    write!(f, "          URL: \"{}\"", url)?;
                                }
                            }
                        }
                    }
                }
                | _ => {
                    // For other frame types not yet enhanced, show basic info
                    if let Some(text) = self.get_text() {
                        if !text.is_empty() {
                            write!(f, " - Text: \"{}\"", text.chars().take(50).collect::<String>())?;
                            if text.len() > 50 {
                                write!(f, "...")?;
                            }
                        }
                    } else if let Some(url) = self.get_url() {
                        if !url.is_empty() {
                            write!(f, " - URL: \"{}\"", url)?;
                        }
                    }
                }
            }
        } else {
            // Fallback for unparsed content
            if let Some(text) = self.get_text() {
                if !text.is_empty() {
                    write!(f, " - Text: \"{}\"", text.chars().take(50).collect::<String>())?;
                    if text.len() > 50 {
                        write!(f, "...")?;
                    }
                }
            } else if let Some(url) = self.get_url() {
                if !url.is_empty() {
                    write!(f, " - URL: \"{}\"", url)?;
                }
            }
        }

        if let Some(embedded) = &self.embedded_frames {
            if !embedded.is_empty() {
                write!(f, "\n    {} embedded sub-frame(s)", embedded.len())?;
            }
        }

        writeln!(f)?; // Add newline at the end of frame display
        writeln!(f)?; // Add blank line for better separation between frames
        Ok(())
    }
}
