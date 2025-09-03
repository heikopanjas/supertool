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
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
    writeln!(&mut stdout, "  Raw header bytes: {:02X?}", id3_header)?;
    stdout.reset()?;

    // Calculate tag size (synchsafe integer)
    let size = decode_synchsafe_int(&id3_header[6..10]);

    // Add diagnostic for size bytes and calculation
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "  Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}]", id3_header[6], id3_header[7], id3_header[8], id3_header[9])?;
    writeln!(
        &mut stdout,
        "  Size calculation: ({} << 21) | ({} << 14) | ({} << 7) | {} = {}",
        id3_header[6] & 0x7F,
        id3_header[7] & 0x7F,
        id3_header[8] & 0x7F,
        id3_header[9] & 0x7F,
        size
    )?;
    stdout.reset()?;

    // Validate synchsafe format (each byte should have MSB = 0)
    let mut synchsafe_violation = false;
    for (i, &byte) in id3_header[6..10].iter().enumerate() {
        if byte & 0x80 != 0 {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
            writeln!(&mut stdout, "  WARNING: Size byte {} (0x{:02X}) violates synchsafe format (MSB set)!", i, byte)?;
            stdout.reset()?;
            synchsafe_violation = true;
        }
    }

    if synchsafe_violation {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        writeln!(&mut stdout, "  ERROR: Invalid synchsafe format detected in size field")?;
        stdout.reset()?;
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
