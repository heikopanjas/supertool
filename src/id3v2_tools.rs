use std::fs::File;
use std::io::Write;
use std::io::{Read, Seek, SeekFrom};

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

    // Add diagnostic output for raw header bytes
    println!("  Raw header bytes: {:02X?}", id3_header);

    // Calculate tag size (synchsafe integer)
    let size = decode_synchsafe_int(&id3_header[6..10]);

    // Add diagnostic for size bytes and calculation
    println!("  Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}]", id3_header[6], id3_header[7], id3_header[8], id3_header[9]);
    println!(
        "  Size calculation: ({} << 21) | ({} << 14) | ({} << 7) | {} = {}",
        id3_header[6] & 0x7F,
        id3_header[7] & 0x7F,
        id3_header[8] & 0x7F,
        id3_header[9] & 0x7F,
        size
    );

    // Validate synchsafe format (each byte should have MSB = 0)
    let mut synchsafe_violation = false;
    for (i, &byte) in id3_header[6..10].iter().enumerate() {
        if byte & 0x80 != 0 {
            println!("  WARNING: Size byte {} (0x{:02X}) violates synchsafe format (MSB set)!", i, byte);
            synchsafe_violation = true;
        }
    }

    if synchsafe_violation {
        println!("  ERROR: Invalid synchsafe format detected in size field");
    }

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

/// Check if a frame ID is valid for ID3v2.3
pub fn is_valid_id3v2_3_frame(frame_id: &str) -> bool {
    const VALID_ID3V2_3_FRAME_IDS: &[&str] = &[
        // Text information frames
        "TALB", "TBPM", "TCOM", "TCON", "TCOP", "TDAT", "TDLY", "TENC", "TEXT", "TFLT", "TIME", "TIT1", "TIT2", "TIT3", "TKEY", "TLAN", "TLEN", "TMED", "TOAL", "TOFN",
        "TOLY", "TOPE", "TORY", "TOWN", "TPE1", "TPE2", "TPE3", "TPE4", "TPOS", "TPUB", "TRCK", "TRDA", "TRSN", "TRSO", "TSIZ", "TSRC", "TSSE", "TYER", "TXXX",
        // URL link frames
        "WCOM", "WCOP", "WOAF", "WOAR", "WOAS", "WORS", "WPAY", "WPUB", "WXXX", // Other frames
        "UFID", "MCDI", "ETCO", "MLLT", "SYTC", "USLT", "SYLT", "COMM", "RVAD", "EQUA", "RVRB", "PCNT", "POPM", "RBUF", "AENC", "LINK", "POSS", "USER", "OWNE",
        "COMR", "ENCR", "GRID", "PRIV", "GEOB", "IPLS", "APIC", // Chapter frames (ID3v2 Chapter Frame Addendum)
        "CHAP", "CTOC",
    ];

    VALID_ID3V2_3_FRAME_IDS.contains(&frame_id)
}

/// Check if a frame ID is valid for ID3v2.4
pub fn is_valid_id3v2_4_frame(frame_id: &str) -> bool {
    const VALID_ID3V2_4_FRAME_IDS: &[&str] = &[
        // Text information frames
        "TALB", "TBPM", "TCOM", "TCON", "TCOP", "TDEN", "TDLY", "TDOR", "TDRC", "TDRL", "TDTG", "TENC", "TEXT", "TFLT", "TIPL", "TIT1", "TIT2", "TIT3", "TKEY", "TLAN",
        "TLEN", "TMCL", "TMED", "TMOO", "TOAL", "TOFN", "TOLY", "TOPE", "TOWN", "TPE1", "TPE2", "TPE3", "TPE4", "TPOS", "TPRO", "TPUB", "TRCK", "TRSN", "TRSO",
        "TSOA", "TSOP", "TSOT", "TSRC", "TSSE", "TSST", "TXXX", // URL link frames
        "WCOM", "WCOP", "WOAF", "WOAR", "WOAS", "WORS", "WPAY", "WPUB", "WXXX", // Other frames
        "UFID", "MCDI", "ETCO", "MLLT", "SYTC", "USLT", "SYLT", "COMM", "RVA2", "EQU2", "RVRB", "PCNT", "POPM", "RBUF", "AENC", "LINK", "POSS", "USER", "OWNE",
        "COMR", "ENCR", "GRID", "PRIV", "GEOB", "APIC", "SEEK", "ASPI", "SIGN", // Chapter frames (ID3v2 Chapter Frame Addendum)
        "CHAP", "CTOC",
    ];

    VALID_ID3V2_4_FRAME_IDS.contains(&frame_id)
}

/// Check if a frame ID is valid for a specific ID3v2 version
pub fn is_valid_frame_for_version(frame_id: &str, version_major: u8) -> bool {
    match version_major {
        | 3 => is_valid_id3v2_3_frame(frame_id),
        | 4 => is_valid_id3v2_4_frame(frame_id),
        | _ => false, // Unsupported version
    }
}

/// Parse embedded frames from raw frame data
/// Used by both CHAP and CTOC frames to parse their embedded sub-frames
pub fn parse_embedded_frames(frame_data: &[u8], version_major: u8) -> Vec<crate::id3v2_frame::Id3v2Frame> {
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
        if !is_valid_frame_for_version(&frame_id, version_major) {
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

        // Create the embedded frame with relative offset within the parent frame
        let mut embedded_frame = crate::id3v2_frame::Id3v2Frame::new_with_offset(frame_id, frame_size, frame_flags, pos, data);

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

/// Display frame header information with customizable indentation
/// This function provides unified frame header display for both top-level and embedded frames
pub fn display_frame_header(output: &mut dyn Write, frame: &crate::id3v2_frame::Id3v2Frame, indentation: &str) -> std::io::Result<()> {
    // Extract the individual bytes from the frame ID for diagnostic display
    let id_bytes = frame.id.as_bytes();
    let size_bytes = frame.size.to_be_bytes();

    // Display frame header information in the same format as top-level frames
    if let Some(offset) = frame.offset {
        writeln!(
            output,
            "{}Frame offset 0x{:08X}, ID bytes = [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] (\"{}\"), Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] = {} bytes, Flags: 0x{:04X}",
            indentation,
            offset,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            frame.id,
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            frame.size,
            frame.flags
        )?;
    } else {
        // Fallback for frames without offset information
        writeln!(
            output,
            "{}ID bytes = [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] (\"{}\"), Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] = {} bytes, Flags: 0x{:04X}",
            indentation,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            frame.id,
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            frame.size,
            frame.flags
        )?;
    }

    Ok(())
}
