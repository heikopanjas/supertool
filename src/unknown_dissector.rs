use crate::media_dissector::MediaDissector;
use std::fs::File;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Fallback dissector for unknown file formats
pub struct UnknownDissector;

impl MediaDissector for UnknownDissector {
    fn media_type(&self) -> &'static str {
        "Unknown"
    }

    fn dissect(&self, _file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(&mut stdout, "Unknown format - no suitable dissector available")?;
        stdout.reset()?;

        Ok(())
    }

    fn can_handle(&self, _header: &[u8]) -> bool {
        true // Always can handle as fallback
    }

    fn name(&self) -> &'static str {
        "Unknown Format Dissector"
    }
}
