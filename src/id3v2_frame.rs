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
    Binary,
}

impl fmt::Display for Id3v2FrameContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            | Id3v2FrameContent::Text(text_frame) => write!(f, "{}", text_frame),
            | Id3v2FrameContent::Url(url_frame) => write!(f, "{}", url_frame),
            | Id3v2FrameContent::UserText(user_text_frame) => write!(f, "{}", user_text_frame),
            | Id3v2FrameContent::UserUrl(user_url_frame) => write!(f, "{}", user_url_frame),
            | Id3v2FrameContent::Comment(comment_frame) => write!(f, "{}", comment_frame),
            | Id3v2FrameContent::Picture(picture_frame) => write!(f, "{}", picture_frame),
            | Id3v2FrameContent::UniqueFileId(ufid_frame) => write!(f, "{}", ufid_frame),
            | Id3v2FrameContent::Chapter(chapter_frame) => write!(f, "{}", chapter_frame),
            | Id3v2FrameContent::TableOfContents(toc_frame) => write!(f, "{}", toc_frame),
            | Id3v2FrameContent::Binary => Ok(()),
        }
    }
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
    /// Frame offset in the file (for top-level frames) or within parent frame (for embedded frames)
    pub offset: Option<usize>,
    /// Raw frame data content
    pub data: Vec<u8>,
    /// Parsed frame content (if successfully parsed)
    pub content: Option<Id3v2FrameContent>,
    /// Embedded sub-frames (for CHAP and CTOC frames)
    pub embedded_frames: Option<Vec<Id3v2Frame>>,
}

impl Id3v2Frame {
    /// Create a new ID3v2 frame with offset information
    pub fn new_with_offset(id: String, size: u32, flags: u16, offset: usize, data: Vec<u8>) -> Self {
        Self { id, size, flags, offset: Some(offset), data, content: None, embedded_frames: None }
    }

    /// Parse frame content based on frame ID
    pub fn parse_content(&mut self, version_major: u8) -> Result<(), String> {
        // Validate that this frame is valid for the given ID3v2 version
        if !crate::id3v2_tools::is_valid_frame_for_version(&self.id, version_major) {
            // Invalid frame for this version, store as binary data
            self.content = Some(Id3v2FrameContent::Binary);
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
            | _ => Id3v2FrameContent::Binary,
        };

        self.content = Some(content);
        Ok(())
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
}

impl fmt::Display for Id3v2Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame: {} ({})", self.id, get_frame_description(&self.id))?;
        write!(f, " - Size: {} bytes", self.size)?;

        if self.flags != 0 {
            write!(f, " - Flags: 0x{:04X}", self.flags)?;
        }

        // Show detailed parsed content using the frame's own Display implementation
        if let Some(content) = &self.content {
            writeln!(f)?;
            // Add 4-space indentation to each line of the frame content
            let content_str = format!("{}", content);
            for line in content_str.lines() {
                if !line.is_empty() {
                    writeln!(f, "    {}", line)?;
                } else {
                    writeln!(f)?;
                }
            }
        } else {
            // Fallback for unparsed content
            if let Some(text) = self.get_text() {
                if !text.is_empty() {
                    write!(f, " - Text: \"{}\"", text)?;
                }
            } else if let Some(url) = self.get_url() {
                if !url.is_empty() {
                    write!(f, " - URL: \"{}\"", url)?;
                }
            }
        }

        if let Some(embedded) = &self.embedded_frames {
            if !embedded.is_empty() {
                writeln!(f, "    {} embedded sub-frame(s)", embedded.len())?;
            }
        }

        writeln!(f)?; // Add newline at the end of frame display
        Ok(())
    }
}
