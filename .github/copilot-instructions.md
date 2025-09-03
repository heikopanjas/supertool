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
- Main entry point: `src/main.rs` (CLI interface and dissector coordination)
- Core modules:
  - `src/media_dissector.rs` - Common trait for all dissectors
  - `src/dissector_builder.rs` - Builder pattern for automatic dissector selection
  - `src/unknown_dissector.rs` - Fallback dissector for unrecognized formats
  - `src/cli.rs` - CLI argument structures and commands
  - `src/id3v2_dissector.rs` - Main ID3v2 header parsing and version dispatch
  - `src/id3v2_3_dissector.rs` - Specialized ID3v2.3 frame dissection
  - `src/id3v2_4_dissector.rs` - Specialized ID3v2.4 frame dissection
  - `src/id3v2_frame.rs` - ID3v2 frame data structure and parsing utilities
  - `src/id3v2_text_encoding.rs` - Text encoding types and decoding utilities for ID3v2 frames
  - `src/id3v2_text_frame.rs` - Text Information Frame (T*** frames except TXXX)
  - `src/id3v2_url_frame.rs` - URL Link Frame (W*** frames except WXXX)
  - `src/id3v2_user_text_frame.rs` - User-Defined Text Information Frame (TXXX)
  - `src/id3v2_user_url_frame.rs` - User-Defined URL Link Frame (WXXX)
  - `src/id3v2_comment_frame.rs` - Comment Frame (COMM, USLT)
  - `src/id3v2_attached_picture_frame.rs` - Attached Picture Frame (APIC)
  - `src/id3v2_unique_file_id_frame.rs` - Unique File Identifier Frame (UFID)
  - `src/id3v2_chapter_frame.rs` - Chapter Frame (CHAP) from ID3v2 Chapter Frame Addendum
  - `src/id3v2_table_of_contents_frame.rs` - Table of Contents Frame (CTOC) from ID3v2 Chapter Frame Addendum
  - `src/id3v2_tools.rs` - Utility functions for ID3v2 processing (synchsafe integers, unsynchronization, frame flags)
  - `src/isobmff_dissector.rs` - ISO Base Media File Format box parsing for MP4 files
- Use Cargo for dependency management and builds
- Follow "one struct/trait per file" organization principle

### Dependencies
- `clap 4.5` with derive features for CLI argument parsing
- `tokio 1.40` with full features for async runtime
- `termcolor 1.4` for cross-platform colored terminal output

### Technical Implementation
- **Common Dissector Trait**: All dissectors implement the `MediaDissector` trait providing unified interface with `dissect()`, `can_handle()`, and metadata methods
- **Dissector Builder Pattern**: `DissectorBuilder` analyzes file headers and returns the appropriate dissector automatically
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
5. **NEVER commit automatically** - only commit when explicitly requested by the user
6. Use conventional commits format for commit messages when requested
7. Update this file when significant architectural decisions are made

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
- **Common dissector trait implementation**: Added `MediaDissector` trait and `DissectorBuilder` pattern
- **Reasoning**: Provides unified interface for all dissector types, enables automatic format detection and dissector selection, makes code more extensible and maintainable following Rust trait-based design patterns
- **Separate ID3v2 dissector implementations**: Moved `MediaDissector` trait implementations to individual dissector files (id3v2_3_dissector.rs, id3v2_4_dissector.rs, isobmff_dissector.rs)
- **Reasoning**: Each dissector now owns its complete implementation including format detection logic, making the codebase more modular and maintainable. Common ID3v2 functionality remains in id3v2_tools.rs for shared use.
- **Modular restructuring completed**: Implemented "one struct/trait per file" organization principle
- **Reasoning**: Split original `dissector.rs` into separate files: `media_dissector.rs` (trait), `dissector_builder.rs` (builder struct), `unknown_dissector.rs` (fallback struct), and `cli.rs` (CLI structures). This follows Rust best practices for maintainable, focused modules with single responsibilities, making the codebase easier to navigate and modify.
- **ID3v2 frame structure implementation**: Created `Id3v2Frame` struct for standardized frame representation
- **Reasoning**: Added dedicated data structure in `id3v2_frame.rs` to encapsulate frame header data (ID, size, flags) and content with version-specific parsing methods. Includes comprehensive frame type descriptions and flag interpretation for both ID3v2.3 and ID3v2.4, providing a clean API for frame manipulation and analysis.
- **Frame struct redesigned for version independence**: Removed version dependency from `Id3v2Frame` struct and moved parsing logic to respective dissectors
- **Reasoning**: Frame structs should be version-agnostic data containers. Moved `parse_id3v2_3_frame()` and `parse_id3v2_4_frame()` functions to their respective dissector modules along with comprehensive lists of valid frame IDs per specification. This separation of concerns makes the frame struct reusable across versions while keeping version-specific logic properly isolated in dissector modules.
- **Frame description centralized**: Moved frame description functionality from `Id3v2Frame` to `id3v2_tools.rs` as unified function
- **Reasoning**: Frame descriptions should be unified across ID3v2 versions rather than duplicated in the frame struct. Added `get_frame_description()` function in `id3v2_tools.rs` that provides human-readable descriptions for all frame types from both ID3v2.3 and ID3v2.4 specifications, creating a single source of truth for frame information.
- **ID3v2 chapter support implementation**: Added comprehensive support for CHAP and CTOC frames from ID3v2 Chapter Frame Addendum
- **Reasoning**: CHAP (Chapter) and CTOC (Table of Contents) frames were missing from the implementation, which prevented proper dissection of audio files with chapter information. Added frame IDs to both ID3v2.3 and ID3v2.4 dissectors, enhanced `Id3v2Frame` struct with `embedded_frames` field to support nested sub-frames in CHAP frames, and implemented parsing functions `parse_chap_frame()` and `parse_ctoc_frame()` that correctly handle the complex structure including element IDs, timing information, flags, child elements, and embedded sub-frames as specified in the ID3v2 Chapter Frame Addendum specification.
- **Frame types modularization**: Split `id3v2_frame_types.rs` into individual files following "one struct/trait per file" principle
- **Reasoning**: Separated large consolidated frame types file into focused modules: `id3v2_text_encoding.rs` (common text encoding utilities), `id3v2_text_frame.rs`, `id3v2_url_frame.rs`, `id3v2_user_text_frame.rs`, `id3v2_user_url_frame.rs`, `id3v2_comment_frame.rs`, `id3v2_attached_picture_frame.rs`, and `id3v2_unique_file_id_frame.rs`. This improves code maintainability, follows Rust best practices for module organization, and makes the codebase easier to navigate and modify. Each frame type now has its own dedicated file with clear responsibilities, while common text encoding functionality is shared through the `id3v2_text_encoding` module.
- **CHAP and CTOC frame types added**: Implemented dedicated modules for Chapter (CHAP) and Table of Contents (CTOC) frames
- **Reasoning**: Added complete support for ID3v2 Chapter Frame Addendum specification with `id3v2_chapter_frame.rs` and `id3v2_table_of_contents_frame.rs` modules. These frame types are essential for audio files with chapter information (podcasts, audiobooks, etc.). CHAP frames contain element ID, timing information, byte offsets, and embedded sub-frames. CTOC frames contain element ID, flags, child element lists, and embedded sub-frames. Both frame types include proper embedded frame parsing that handles different ID3v2 versions (synchsafe vs regular integers, different header sizes). Integration includes adding variants to `Id3v2FrameContent` enum and parsing logic in `parse_content()` method.
- **Modular ID3v2 frame type system completed**: Implemented comprehensive frame parsing system with dedicated modules following "one struct/trait per file" principle
- **Reasoning**: Created complete modular architecture with `Id3v2Frame` struct containing `Id3v2FrameContent` enum for all supported frame types. Each frame type has its own dedicated module with specialized parsing logic: text frames, URL frames, user-defined frames, comments, pictures, unique file IDs, and chapter frames. Added unified text encoding system in `id3v2_text_encoding.rs` for consistent text handling across all frame types. Integrated structured frame parsing into both ID3v2.3 and ID3v2.4 dissectors, replacing raw content preview with properly parsed and formatted frame information. This provides clean separation of concerns, type safety, and extensibility for future frame type additions.
- **Frame-specific logic encapsulation**: Moved embedded frame parsing from `id3v2_tools.rs` to respective frame modules
- **Reasoning**: Refactored `parse_embedded_frames()` function from the generic `id3v2_tools.rs` into `ChapterFrame::parse_embedded_frames()` and `TableOfContentsFrame::parse_embedded_frames()` methods. This makes `id3v2_tools.rs` truly frame-type and version-agnostic, containing only core utilities like synchsafe integer decoding, unsynchronization removal, and frame flag interpretation. Frame-specific parsing logic now belongs in the appropriate frame modules, following proper separation of concerns and improving code maintainability.
- **Comprehensive diagnostic system implemented**: Added detailed error reporting and diagnostic output throughout the parsing pipeline
- **Reasoning**: Enhanced `id3v2_dissector.rs`, `id3v2_3_dissector.rs`, and `id3v2_tools.rs` with comprehensive diagnostic output including raw byte inspection, synchsafe integer validation, frame parsing status, and error reporting. Added validation for synchsafe format violations, size sanity checks, and detailed frame-by-frame parsing diagnostics. This enables identification of parsing issues, corrupted files, and specification violations. Diagnostics include color-coded output for different message types (errors, warnings, info) and summary statistics for parsed frames, errors, and unprocessed bytes. Essential for debugging sample files with large or unusual tag structures.
- **Podcast-aware size limits implemented**: Adjusted tag size limits to accommodate real-world podcast content with chapter images
- **Reasoning**: Increased tag size limits from 10MB to 100MB hard limit to support podcast MP3s with embedded images in CHAP frames. Modern podcasts can have dozens of chapters each with embedded artwork, easily resulting in 20-50MB+ ID3v2 tags. Added tiered warning system (10MB = info, 50MB = warning, 100MB = error) and enhanced statistics showing chapter count, image count, total image size, and large frame detection. This ensures the tool works with legitimate large podcast files while still detecting truly corrupted data. Addresses real-world usage patterns where podcast publishers embed chapter-specific images.
