# Copilot Instructions for Supertool

**Last updated:** September 3, 2025

## Project Overview
This is a Rust project called "supertool" - a versatile media file analysis tool that dissects ID3v2 tags (MP3 files) and ISO Base Media File Format containers (MP4 files). The project runs on macOS, Windows, and Linux with a modular architecture and CLI interface.

## Development Guidelines

### Code Style & Standards
- Follow Rust best practices and idioms
- Ensure cross-platform compatibility (macOS, Windows, Linux)
- Use `rustfmt` for code formatting
- Run `clippy` for linting and suggestions
- Write clear, descriptive commit messages using conventional commits format

### Project Structure
- Source code in `src/`
- Main entry point: `src/main.rs` (CLI interface and file detection)
- Core modules:
  - `src/id3v2_dissector.rs` - Main ID3v2 header parsing and version dispatch
  - `src/id3v2_3_dissector.rs` - Specialized ID3v2.3 frame dissection
  - `src/id3v2_4_dissector.rs` - Specialized ID3v2.4 frame dissection
  - `src/id3v2_tools.rs` - Utility functions for ID3v2 processing (synchsafe integers, unsynchronization, frame flags)
  - `src/isobmff_dissector.rs` - ISO Base Media File Format box parsing for MP4 files
- Use Cargo for dependency management and builds

### Dependencies
- `clap 4.5` with derive features for CLI argument parsing
- `tokio 1.40` with full features for async runtime
- `termcolor 1.4` for cross-platform colored terminal output

### Technical Implementation
- **ID3v2 Support**: Specification-compliant parsing for ID3v2.3 and ID3v2.4 with proper unsynchronization handling, frame flag interpretation, and UTF-16 text support
- **ISO BMFF Support**: Box header parsing with size and type detection for MP4 containers
- **File Format Detection**: Automatic detection based on file headers (ID3 tags, MPEG sync patterns, ftyp boxes)
- **CLI Interface**: Subcommand-based interface with `dissect` command for file analysis
- **Cross-Platform**: Windows, macOS, and Linux compatibility with proper terminal color support

### Documentation
- Document public APIs with rustdoc comments
- Keep README updated with project status and usage
- Maintain this copilot instructions file as the project evolves

## Development Workflow
1. Make changes following the guidelines above
2. Test changes with `cargo run -- dissect <file>` to test file dissection
3. Run `cargo build` to ensure compilation
4. Use `cargo run -- --help` to verify CLI interface
5. Use conventional commits format for commit messages
6. Update this file when significant architectural decisions are made

### Important Notes
- Use terminology "dissect" rather than "parse" for media analysis operations
- Prefer "ID3v2" over "MP3" and "ISO BMFF" over "MP4" for technical accuracy
- Maintain specification compliance for ID3v2.3/2.4 and ISO Base Media File Format standards

---

## Recent Updates & Decisions

### 2025-09-03
- **Initial setup**: Created initial copilot instructions file for new Rust project
- **Reasoning**: Establishing development standards and workflow from the beginning of the project
- **Cross-platform requirement**: Added multi-platform compatibility requirement (macOS, Windows, Linux)
- **Reasoning**: Ensuring supertool works across all major desktop operating systems
- **Core architecture implementation**: Built modular architecture with separate dissector modules
- **Reasoning**: Separation of concerns between ID3v2 and ISO BMFF parsing logic
- **ID3v2 parser fixes**: Fixed critical issues in MP3 ID3v2 parsing implementation
- **Reasoning**: Aligned implementation with official ID3v2.3/2.4 specifications for accurate parsing
- **Terminology precision**: Renamed "parser" to "dissector" throughout codebase
- **Reasoning**: "Dissector" better reflects the analysis nature of the tool
- **Module structure finalized**:
  - `id3v2_dissector.rs` (ID3v2 header parsing and version dispatch)
  - `id3v2_3_dissector.rs` (specialized ID3v2.3 frame dissection)
  - `id3v2_4_dissector.rs` (specialized ID3v2.4 frame dissection)
  - `isobmff_dissector.rs` (ISO BMFF box parsing)
  - `id3v2_tools.rs` (utility functions for synchsafe integers, unsynchronization)
- **Reasoning**: Clean separation allows for maintainable, testable code with clear responsibilities
- **CLI interface completed**: Subcommand-based interface with `dissect` command
- **Reasoning**: Professional tool structure that can be extended with additional commands
- **ID3v2 dissector split**: Separated version-specific frame parsing into dedicated modules
- **Reasoning**: ID3v2.3 and ID3v2.4 have different parsing requirements (big-endian vs synchsafe integers, different frame flags), splitting improves code clarity and maintainability
