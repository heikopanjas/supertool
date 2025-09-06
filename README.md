# Supertool

A versatile media file analysis tool that dissects ID3v2 tags (MP3 files) and ISO Base Media File Format containers (MP4 files). Built in Rust for cross-platform compatibility with a focus on detailed diagnostic output and specification compliance.

## Features

### ID3v2 Support

- **Complete ID3v2.3 and ID3v2.4 dissection** with specification compliance
- **Rich frame parsing** for all major frame types (TEXT, URL, COMM, APIC, UFID, etc.)
- **Chapter frame support** (CHAP/CTOC) from ID3v2 Chapter Frame Addendum
- **Embedded frame analysis** within chapter structures
- **Diagnostic output** with hex byte inspection and frame validation
- **Large tag handling** optimized for podcast files with chapter images (up to 100MB)

### ISO Base Media File Format Support

- **MP4 container analysis** with box header parsing
- **Size and type detection** for ISO BMFF boxes
- **Cross-platform compatibility** for various MP4 variants

### Advanced Features

- **Automatic format detection** based on file headers
- **Modular architecture** with pluggable dissector system
- **Colored diagnostic output** for enhanced readability
- **Granular output control** with `--header`, `--frames`, and `--all` options
- **Comprehensive error reporting** with detailed validation

## Installation

### Prerequisites

- Rust 1.70+ (stable toolchain)
- Cargo package manager

### From Source

```bash
git clone https://github.com/yourusername/supertool.git
cd supertool
cargo build --release
```

The binary will be available at `target/release/supertool`.

### Development Build

```bash
cargo build
```

## Usage

### Basic Analysis

```bash
# Analyze an MP3 file with full output
supertool debug song.mp3

# Analyze an MP4 file
supertool debug video.mp4
```

### Granular Output Control

```bash
# Show only header information
supertool debug --header podcast.mp3

# Show only frames/content
supertool debug --frames audiobook.mp3

# Show everything (default)
supertool debug --all music.mp3
```

### Command Reference

```text
supertool debug [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to the media file to analyze

Options:
      --header  Show only header information (ID3v2/ISO BMFF header)
      --frames  Show only frames/boxes information
      --all     Show both header and frames/boxes (default if no options specified)
  -h, --help    Print help
```

## Sample Output

### ID3v2 Analysis

```text
Analyzing file: samples/podcast.mp3
Detected format: ID3v2.4 (ID3v2.4 Dissector)

ID3v2 Header Found:
  Version: 2.4.0
  Flags: 0x00
  Tag Size: 15458392 bytes
  INFO: Large tag size (> 10MB), possibly podcast with embedded chapter content

ID3v2.4 Frames:
    Frame offset 0x0000000A, ID: [0x54, 0x49, 0x54, 0x32] = "TIT2", Size: [0x00, 0x00, 0x00, 0x2D] = 45, Flags: 0x0000
    TIT2 (Title/songname/content description) - Size: 45 bytes
        Encoding: UTF-8
        Value: "Episode 123: The Future of Technology"

    Frame offset 0x00000047, ID: [0x43, 0x48, 0x41, 0x50] = "CHAP", Size: [0x00, 0x05, 0x6C, 0xE3] = 355555, Flags: 0x0000
    CHAP (Chapter frame) - Size: 355555 bytes
        Element ID: "chapter_001"
        Time: 00:00:00.000 - 00:15:30.500 (duration: 00:15:30.500)
        Sub-frames: 3 embedded frame(s)

            Frame: TIT2 (Title/songname/content description) - Size: 28 bytes
                Encoding: UTF-8
                Value: "Introduction"

            Frame: APIC (Attached picture) - Size: 342856 bytes
                Encoding: UTF-8
                MIME type: "image/jpeg"
                Picture type: Cover (front)
                Description: "Chapter 1 Cover"
                Data size: 342825 bytes
```

### Header-Only Output

```bash
supertool debug --header podcast.mp3
```

```text
Analyzing file: podcast.mp3
Detected format: ID3v2.4 (ID3v2.4 Dissector)

ID3v2 Header Found:
  Version: 2.4.0
  Flags: 0x00
  Tag Size: 15458392 bytes
  INFO: Large tag size (> 10MB), possibly podcast with embedded chapter content
```

## Supported Formats

### ID3v2 Tags

- **ID3v2.3** - Complete frame parsing with big-endian integers
- **ID3v2.4** - Full support with synchsafe integers and extended features
- **Chapter Frames** - CHAP and CTOC from ID3v2 Chapter Frame Addendum
- **All standard frames** - TEXT, URL, COMM, APIC, UFID, TXXX, WXXX, etc.

### Media Containers

- **MP3** - MPEG audio files with ID3v2 tags
- **MP4** - ISO Base Media File Format containers
- **M4A** - Audio-only MP4 containers

## Technical Details

### Architecture

- **Modular Design** - Pluggable dissector system with trait-based architecture
- **Format Detection** - Automatic dissector selection based on file headers
- **Memory Efficient** - Streaming analysis without loading entire files
- **Error Resilient** - Graceful handling of corrupted or non-standard files

### Frame Types Supported

- **Text Frames** (T***) - All standard text information frames
- **URL Frames** (W***) - Web link frames with descriptions
- **Comment Frames** (COMM, USLT) - Comments and unsynchronized lyrics
- **Picture Frames** (APIC) - Embedded artwork with type descriptions
- **Chapter Frames** (CHAP, CTOC) - Podcast/audiobook chapter structures
- **User-Defined Frames** (TXXX, WXXX) - Custom text and URL frames
- **Unique ID Frames** (UFID) - File identification frames

### Specifications Compliance

- **ID3v2.3** - Full compliance with original specification
- **ID3v2.4** - Complete implementation including synchsafe integers
- **ID3v2 Chapter Addendum** - CHAP and CTOC frame support
- **ISO BMFF** - Basic box parsing for MP4 containers

## Development

### Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint with clippy
cargo clippy
```

## Dependencies

- **clap 4.5** - Command-line argument parsing with derive features
- **owo-colors 4.1** - Enhanced colored terminal output

### Development Guidelines

- Follow Rust best practices and idioms
- Maintain cross-platform compatibility
- Use "dissect" terminology instead of "parse"
- Add comprehensive diagnostic output
- Update documentation for new features

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- ID3v2 specification maintainers for comprehensive documentation
- ISO for the Base Media File Format specification
- Rust community for excellent tooling and libraries
- Podcast creators whose files helped test large tag handling

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

---

**Note**: This tool is designed for analysis and debugging purposes. It provides detailed diagnostic information about media file structures and metadata without modifying the original files.
