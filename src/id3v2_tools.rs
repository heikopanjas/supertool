use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

/// Parse and display a preview of frame content for common text frames
pub fn parse_frame_content_preview(data: &[u8], frame_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    if data.is_empty() {
        return Ok(());
    }

    // Text frames typically start with encoding byte
    if matches!(frame_id, "TIT2" | "TPE1" | "TALB" | "TYER" | "TCON") {
        let encoding = data[0];
        if data.len() > 1 {
            let text_data = &data[1..];
            let preview = match encoding {
                | 0 => {
                    // ISO-8859-1
                    let end = text_data.iter().position(|&b| b == 0).unwrap_or(text_data.len());
                    String::from_utf8_lossy(&text_data[..end.min(50)])
                }
                | 1 => {
                    // UTF-16 with BOM
                    if text_data.len() >= 2 {
                        let (text_start, is_le) = if text_data.len() >= 2 {
                            if text_data[0] == 0xFF && text_data[1] == 0xFE {
                                (2, true) // Little endian
                            } else if text_data[0] == 0xFE && text_data[1] == 0xFF {
                                (2, false) // Big endian
                            } else {
                                (0, false) // No BOM, assume big endian
                            }
                        } else {
                            (0, false)
                        };

                        if text_data.len() > text_start {
                            // Simple UTF-16 to string conversion for preview
                            let utf16_data = &text_data[text_start..];
                            let mut preview = String::new();
                            let mut i = 0;
                            while i + 1 < utf16_data.len() && preview.len() < 50 {
                                let code_point = if is_le {
                                    u16::from_le_bytes([utf16_data[i], utf16_data[i + 1]])
                                } else {
                                    u16::from_be_bytes([utf16_data[i], utf16_data[i + 1]])
                                };

                                if code_point == 0 {
                                    break;
                                }
                                if code_point < 128 && code_point > 0 {
                                    preview.push(code_point as u8 as char);
                                } else {
                                    preview.push('?'); // Non-ASCII placeholder
                                }
                                i += 2;
                            }
                            preview.into()
                        } else {
                            "".into()
                        }
                    } else {
                        "".into()
                    }
                }
                | 3 => {
                    // UTF-8
                    let end = text_data.iter().position(|&b| b == 0).unwrap_or(text_data.len());
                    String::from_utf8_lossy(&text_data[..end.min(50)])
                }
                | _ => format!("Unknown encoding: {}", encoding).into(),
            };

            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            writeln!(&mut stdout, "    Content: \"{}\"", preview.trim())?;
            stdout.reset()?;
        }
    }

    Ok(())
}
