/// URL Link Frame (W*** frames except WXXX)
///
/// Structure: URL (text string)
/// Examples: WCOM, WCOP, WOAF, WOAR, WOAS, WORS, WPAY, WPUB
use crate::id3v2_text_encoding::decode_iso88591_string;

#[derive(Debug, Clone)]
pub struct UrlFrame {
    pub url: String,
}

impl UrlFrame {
    /// Parse a URL frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        // URL frames are always encoded in ISO-8859-1
        let url = decode_iso88591_string(data);
        Ok(UrlFrame { url })
    }
}
