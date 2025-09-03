use std::fs::File;
use std::io::Write;
use std::io::{Read, Seek, SeekFrom};
use termcolor::{ColorChoice, StandardStream};

/// ID3v2 header information: (major_version, minor_version, flags, size)
pub type Id3v2Header = (u8, u8, u8, u32);

/// Get a human-readable description for an ID3v2 frame ID (unified for v2.3 and v2.4)
pub fn get_frame_description(frame_id: &str) -> &'static str {
    match frame_id {
        | "TIT1" => "Content group description",
        | "TIT2" => "Title/songname/content description",
        | "TIT3" => "Subtitle/Description refinement",
        | "TALB" => "Album/Movie/Show title",
        | "TOAL" => "Original album/movie/show title",
        | "TRCK" => "Track number/Position in set",
        | "TPOS" => "Part of a set",
        | "TSST" => "Set subtitle",
        | "TSRC" => "ISRC (international standard recording code)",
        | "TPE1" => "Lead performer(s)/Soloist(s)",
        | "TPE2" => "Band/orchestra/accompaniment",
        | "TPE3" => "Conductor/performer refinement",
        | "TPE4" => "Interpreted, remixed, or otherwise modified by",
        | "TOPE" => "Original artist(s)/performer(s)",
        | "TEXT" => "Lyricist/Text writer",
        | "TOLY" => "Original lyricist(s)/text writer(s)",
        | "TCOM" => "Composer",
        | "TMCL" => "Musician credits list",
        | "TIPL" => "Involved people list",
        | "TENC" => "Encoded by",
        | "TBPM" => "BPM (beats per minute)",
        | "TLEN" => "Length",
        | "TKEY" => "Initial key",
        | "TLAN" => "Language(s)",
        | "TCON" => "Content type",
        | "TFLT" => "File type",
        | "TMED" => "Media type",
        | "TMOO" => "Mood",
        | "TCOP" => "Copyright message",
        | "TPRO" => "Produced notice",
        | "TPUB" => "Publisher",
        | "TOWN" => "File owner/licensee",
        | "TRSN" => "Internet radio station name",
        | "TRSO" => "Internet radio station owner",
        | "TOFN" => "Original filename",
        | "TDLY" => "Playlist delay",
        | "TDEN" => "Encoding time",
        | "TDOR" => "Original release time",
        | "TDRC" => "Recording time",
        | "TDRL" => "Release time",
        | "TDTG" => "Tagging time",
        | "TSSE" => "Software/Hardware and settings used for encoding",
        | "TSOA" => "Album sort order",
        | "TSOP" => "Performer sort order",
        | "TSOT" => "Title sort order",
        | "TXXX" => "User defined text information frame",

        // ID3v2.3 specific frames
        | "TDAT" => "Date",
        | "TIME" => "Time",
        | "TORY" => "Original release year",
        | "TRDA" => "Recording dates",
        | "TSIZ" => "Size",
        | "TYER" => "Year",
        | "IPLS" => "Involved people list",
        | "RVAD" => "Relative volume adjustment",
        | "EQUA" => "Equalisation",

        // ID3v2.4 specific frames
        | "RVA2" => "Relative volume adjustment (2)",
        | "EQU2" => "Equalisation (2)",
        | "SEEK" => "Seek frame",
        | "ASPI" => "Audio seek point index",
        | "SIGN" => "Signature frame",

        // URL frames
        | "WCOM" => "Commercial information",
        | "WCOP" => "Copyright/Legal information",
        | "WOAF" => "Official audio file webpage",
        | "WOAR" => "Official artist/performer webpage",
        | "WOAS" => "Official audio source webpage",
        | "WORS" => "Official internet radio station homepage",
        | "WPAY" => "Payment",
        | "WPUB" => "Publishers official webpage",
        | "WXXX" => "User defined URL link frame",

        // Other frames
        | "MCDI" => "Music CD identifier",
        | "ETCO" => "Event timing codes",
        | "MLLT" => "MPEG location lookup table",
        | "SYTC" => "Synchronized tempo codes",
        | "USLT" => "Unsychronized lyric/text transcription",
        | "SYLT" => "Synchronized lyric/text",
        | "COMM" => "Comments",
        | "RVRB" => "Reverb",
        | "PCNT" => "Play counter",
        | "POPM" => "Popularimeter",
        | "RBUF" => "Recommended buffer size",
        | "AENC" => "Audio encryption",
        | "LINK" => "Linked information",
        | "POSS" => "Position synchronisation frame",
        | "USER" => "Terms of use",
        | "OWNE" => "Ownership frame",
        | "COMR" => "Commercial frame",
        | "ENCR" => "Encryption method registration",
        | "GRID" => "Group identification registration",
        | "PRIV" => "Private frame",
        | "GEOB" => "General encapsulated object",
        | "UFID" => "Unique file identifier",
        | "APIC" => "Attached picture",

        // Chapter frames (ID3v2 Chapter Frame Addendum)
        | "CHAP" => "Chapter frame",
        | "CTOC" => "Table of contents frame",

        | _ => "Unknown frame type",
    }
}

/// Check if the given header indicates an ID3v2 file and return the version
pub fn detect_id3v2_version(header: &[u8]) -> Option<(u8, u8)> {
    if header.len() >= 5 && header[0..3] == [0x49, 0x44, 0x33] {
        // "ID3" found
        let major_version = header[3];
        let minor_version = header[4];
        return Some((major_version, minor_version));
    }
    None
}

/// Check if the given header indicates an MPEG file (which might contain ID3v2)
pub fn detect_mpeg_sync(header: &[u8]) -> bool {
    // Check for MPEG sync pattern (0xFF followed by 0xFB, 0xFA, 0xF3, 0xF2)
    if header.len() >= 2 && header[0] == 0xFF && (header[1] & 0xE0) == 0xE0 {
        return true;
    }
    false
}

/// Read and parse ID3v2 header, returning version info and tag size
pub fn read_id3v2_header(file: &mut File) -> Result<Option<Id3v2Header>, Box<dyn std::error::Error>> {
    // Seek to beginning and read ID3v2 header
    file.seek(SeekFrom::Start(0))?;
    let mut id3_header = [0u8; 10];

    if file.read_exact(&mut id3_header).is_err() {
        return Ok(None);
    }

    if &id3_header[0..3] != b"ID3" {
        return Ok(None);
    }

    let version_major = id3_header[3];
    let version_minor = id3_header[4];
    let flags = id3_header[5];

    // Calculate tag size (synchsafe integer)
    let size = decode_synchsafe_int(&id3_header[6..10]);

    Ok(Some((version_major, version_minor, flags, size)))
}

/// Decode a synchsafe integer (7 bits per byte) as used in ID3v2
pub fn decode_synchsafe_int(bytes: &[u8]) -> u32 {
    if bytes.len() >= 4 {
        ((bytes[0] & 0x7F) as u32) << 21 | ((bytes[1] & 0x7F) as u32) << 14 | ((bytes[2] & 0x7F) as u32) << 7 | (bytes[3] & 0x7F) as u32
    } else {
        0
    }
}

/// Remove unsynchronization bytes (0xFF 0x00 -> 0xFF) from ID3v2 data
pub fn remove_unsynchronization(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        result.push(data[i]);

        // If we find 0xFF followed by 0x00, remove the 0x00
        if data[i] == 0xFF && i + 1 < data.len() && data[i + 1] == 0x00 {
            i += 2; // Skip the 0x00 byte
        } else {
            i += 1;
        }
    }

    result
}

/// Interpret ID3v2.3 frame flags and display them
pub fn interpret_id3v2_3_frame_flags(flags: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    if flags != 0 {
        write!(&mut stdout, ", flags: ")?;
        let mut flag_parts = Vec::new();

        // Status message flags (first byte)
        if flags & 0x8000 != 0 {
            flag_parts.push("tag_alter_preserve");
        }
        if flags & 0x4000 != 0 {
            flag_parts.push("file_alter_preserve");
        }
        if flags & 0x2000 != 0 {
            flag_parts.push("read_only");
        }

        // Format description flags (second byte)
        if flags & 0x0080 != 0 {
            flag_parts.push("compressed");
        }
        if flags & 0x0040 != 0 {
            flag_parts.push("encrypted");
        }
        if flags & 0x0020 != 0 {
            flag_parts.push("grouped");
        }

        if flag_parts.is_empty() {
            write!(&mut stdout, "0x{:04X}", flags)?;
        } else {
            write!(&mut stdout, "{}", flag_parts.join("|"))?;
        }
    }

    Ok(())
}

/// Interpret ID3v2.4 frame flags and display them
pub fn interpret_id3v2_4_frame_flags(flags: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    if flags != 0 {
        write!(&mut stdout, ", flags: ")?;
        let mut flag_parts = Vec::new();

        // Status message flags (first byte)
        if flags & 0x4000 != 0 {
            flag_parts.push("tag_alter_preserve");
        }
        if flags & 0x2000 != 0 {
            flag_parts.push("file_alter_preserve");
        }
        if flags & 0x1000 != 0 {
            flag_parts.push("read_only");
        }

        // Format description flags (second byte)
        if flags & 0x0040 != 0 {
            flag_parts.push("grouped");
        }
        if flags & 0x0008 != 0 {
            flag_parts.push("compressed");
        }
        if flags & 0x0004 != 0 {
            flag_parts.push("encrypted");
        }
        if flags & 0x0002 != 0 {
            flag_parts.push("unsynchronised");
        }
        if flags & 0x0001 != 0 {
            flag_parts.push("data_length_indicator");
        }

        if flag_parts.is_empty() {
            write!(&mut stdout, "0x{:04X}", flags)?;
        } else {
            write!(&mut stdout, "{}", flag_parts.join("|"))?;
        }
    }

    Ok(())
}

/// Parse embedded frames from CHAP or CTOC frame data
/// Returns (parsed_data_length, embedded_frames)
pub fn parse_embedded_frames(frame_data: &[u8], start_offset: usize, version_major: u8) -> (usize, Vec<crate::id3v2_frame::Id3v2Frame>) {
    let mut embedded_frames = Vec::new();
    let mut pos = start_offset;

    while pos + 10 <= frame_data.len() {
        // Try to parse a sub-frame
        let frame_id = String::from_utf8_lossy(&frame_data[pos..pos + 4]).to_string();

        // Check if we've reached padding or end of data
        if frame_id.starts_with('\0') || !frame_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            break;
        }

        // Parse frame size based on ID3v2 version
        let frame_size = if version_major == 4 {
            // ID3v2.4 uses synchsafe integers
            decode_synchsafe_int(&frame_data[pos + 4..pos + 8])
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
        let embedded_frame = crate::id3v2_frame::Id3v2Frame::new(frame_id, frame_size, frame_flags, data);
        embedded_frames.push(embedded_frame);

        // Move to next frame
        pos += 10 + frame_size as usize;
    }

    (pos - start_offset, embedded_frames)
}
