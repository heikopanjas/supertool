# Copilot Instructions for Supertool

**Last updated:** September 6, 2025

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
- **Removed obsolete ID3v2 dissector module**: Deleted `src/id3v2_dissector.rs` file and updated module structure
- **Reasoning**: The `id3v2_dissector.rs` module was no longer needed with the current architecture where ID3v2.3 and ID3v2.4 dissectors handle their own parsing logic independently. Removing this file simplifies the module structure and eliminates redundant code. The current architecture with separate `id3v2_3_dissector.rs` and `id3v2_4_dissector.rs` modules provides cleaner separation of version-specific logic without needing a central dispatch module.
- **Enhanced COMM and USLT frame display**: Added rich frame data display for Comment frames (COMM and USLT)
- **Reasoning**: Comment frames now display detailed parsed information similar to TEXT frame display, including encoding, language code, description, and text content. This provides consistent formatting across frame types and better visibility into comment frame structure. The display truncates long text content (>100 characters) with ellipsis for readability while preserving the language and description fields that are unique to comment frames.
- **Fixed frame display formatting**: Added missing newline at the end of frame display output
- **Reasoning**: Frame display output was missing a trailing newline, causing diagnostic messages from the dissector to appear on the same line as frame information. Added `writeln!(f)?;` at the end of the `Display` implementation for `Id3v2Frame` to ensure proper line separation and improve output readability.
- **Enhanced frame visual separation**: Added blank line after each frame display for better readability
- **Reasoning**: Added an additional `writeln!(f)?;` to create visual separation between frame displays and diagnostic output, making it easier to distinguish individual frames in the output and improving overall readability of the dissector results.
- **Rich display for CHAP and CTOC frames**: Implemented detailed display formatting for Chapter and Table of Contents frames
- **Reasoning**: Added comprehensive display support for CHAP (Chapter) and CTOC (Table of Contents) frames in the `Display` trait implementation for `Id3v2Frame`. CHAP frames now show element ID, time range with calculated duration, byte offsets (when used), and embedded sub-frames with descriptions. CTOC frames display element ID, flags (top-level and ordered status), numbered child element lists, and embedded sub-frames. This provides rich, human-readable output for podcast chapter information, making the tool much more useful for analyzing audio files with chapter metadata. The display includes proper formatting with indentation and truncation for long content.
- **Enhanced CHAP timestamp formatting**: Updated CHAP frame time display to use human-readable 'hh:mm:ss.ms' format instead of raw milliseconds
- **Reasoning**: Added `format_timestamp()` helper function that converts milliseconds to 'hh:mm:ss.ms' format for better readability. CHAP frames now display start time, end time, and duration in formats like "00:32:22.877 - 01:01:36.586 (duration: 00:29:13.709)" instead of raw milliseconds. This makes chapter timing information much more accessible to users analyzing podcast and audiobook files, allowing them to easily understand chapter structure and navigate content. The formatting properly handles hours, minutes, seconds, and milliseconds with zero-padding for consistent display.
- **Rich embedded frame display**: Enhanced CHAP and CTOC frames to show detailed information for embedded sub-frames
- **Reasoning**: Added comprehensive rich display for embedded frames within CHAP and CTOC frames, reusing the existing frame display implementation for consistency. The enhancement includes proper content parsing of embedded frames and specialized display for different frame types: TEXT frames show encoding and values, APIC frames display MIME type, picture type with description, and data size in bytes, URL frames show URLs with descriptions, and COMMENT frames display language and text content. This provides users with complete visibility into chapter metadata including embedded artwork, links, and descriptions, making the tool significantly more useful for detailed podcast and audiobook analysis. The implementation ensures embedded frames are parsed with their content during CHAP/CTOC parsing for optimal performance and data availability.
- **Enhanced top-level APIC and UFID frame display**: Added rich data display for top-level APIC and UFID frames
- **Reasoning**: Enhanced the main frame display implementation to show detailed information for APIC (Attached Picture) frames including encoding, MIME type, picture type with human-readable description, optional description text, and data size in bytes. Also added display support for UFID (Unique File Identifier) frames showing owner identifier and identifier data size. This provides consistent rich display for these frame types whether they appear as top-level frames or embedded within chapter frames, improving the tool's usefulness for analyzing media files with cover art and unique identifiers. The display format matches the embedded frame display for consistency.
- **Enhanced embedded TEXT frame display**: Updated embedded TEXT frames to match the comprehensive top-level TEXT frame display format
- **Reasoning**: Enhanced the embedded TEXT frame display in both CHAP and CTOC frames to match the comprehensive top-level TEXT frame display format. The embedded frames now support multiple strings display with proper enumeration, consistent encoding display, proper "Value" labeling, and the same text truncation logic as top-level frames. This provides consistent user experience across all TEXT frame contexts, whether they appear as standalone frames or embedded within chapter structures. The enhancement ensures that complex TEXT frames with multiple values are properly displayed in embedded contexts, maintaining the same level of detail and formatting standards throughout the tool's output.
- **Dead code cleanup**: Removed unused methods and optimized frame content representation
- **Reasoning**: Cleaned up dead code warnings by removing unused constructor methods (`new_with_content`, `new_with_embedded`, `new_complete`) and accessor methods (`id()`, `size()`, `flags()`, `data()`, `is_valid_id()`, `total_size()`, `supports_embedded_frames()`, `embedded_frames()`, `has_embedded_frames()`, `is_parsed()`) from `Id3v2Frame` struct since the fields are public and directly accessible. Also removed unused `all_strings()` method from `TextFrame` and converted `Binary(Vec<u8>)` variant to `Binary` unit variant since the inner data was never accessed (raw data remains available in `Id3v2Frame.data` field). This eliminates compiler warnings while maintaining full functionality and improving code clarity by removing redundant interfaces.

### 2025-09-06
- **Frame display refactoring**: Moved frame-specific display logic from central nested loops to individual frame type implementations
- **Reasoning**: Refactored the large, duplicated display logic in `Id3v2Frame::fmt()` method by implementing `Display` trait for each frame content type (`TextFrame`, `UrlFrame`, `UserTextFrame`, `UserUrlFrame`, `CommentFrame`, `AttachedPictureFrame`, `UniqueFileIdFrame`, `ChapterFrame`, `TableOfContentsFrame`) and `Id3v2FrameContent` enum. Created helper functions `display_embedded_frame_content()` and `format_embedded_display()` in `id3v2_chapter_frame.rs` to handle embedded frame formatting with proper indentation and text truncation. This eliminates the nested loops with nearly identical logic for embedded frames in CHAP and CTOC display, improves code maintainability by putting frame-specific formatting in the appropriate modules, reduces code duplication, and follows the single responsibility principle. The main `Id3v2Frame::fmt()` implementation is now much cleaner and simply delegates to the frame content's own display implementation.
- **Enhanced diagnostic output formatting**: Reformatted frame parsing diagnostic output for improved readability and consistency
- **Reasoning**: Updated diagnostic output in both ID3v2.3 and ID3v2.4 dissectors to display frame information on a single line with comma separation instead of multiple lines. Changed "Frame at position" to "Frame offset" for more precise terminology. Added '0x' prefix to all hexadecimal numbers and applied proper padding based on data size (8-bit values with 2 digits, 16-bit values with 4 digits). The new format "Frame offset {}, ID bytes = [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] (\"{}\"), Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] = {} bytes, Flags: 0x{:04X}" provides all frame header information in a compact, consistent format that's easier to read and parse programmatically. This improves the diagnostic output quality for analyzing MP3 files with complex tag structures.
- **Removed text truncation for COMM and USLT frames**: Disabled text content truncation for comment frames to show complete lyrics and comments
- **Reasoning**: Removed the 100-character truncation limit in `CommentFrame::fmt()` method and simplified the `format_embedded_display()` helper function to not truncate text content for embedded frames. Comment frames (COMM) and unsynchronized lyrics (USLT) often contain important long-form content like full episode descriptions, song lyrics, or detailed comments that users need to see in their entirety. The truncation was hiding valuable content and making the tool less useful for analyzing podcast metadata and song lyrics. Both top-level and embedded comment frames now display their complete text content, improving the tool's utility for content analysis.
- **Display formatting unification**: Unified display formatting between top-level and embedded frames by removing hardcoded indentation from individual frame implementations
- **Reasoning**: Completed comprehensive formatting unification by removing hardcoded 4-space indentation from all frame Display implementations (`TextFrame`, `UrlFrame`, `UserTextFrame`, `UserUrlFrame`, `CommentFrame`, `AttachedPictureFrame`, `UniqueFileIdFrame`, `ChapterFrame`, `TableOfContentsFrame`). Updated the main `Id3v2Frame::fmt()` method to add 4-space indentation when displaying frame content, and modified `format_embedded_display()` helper function to add 10-space indentation for embedded frames. This eliminates the previous inconsistency where embedded frames had different indentation (10 spaces vs 4 spaces for top-level) due to hardcoded indentation plus additional context indentation. The unified approach makes frame Display implementations context-agnostic and allows caller-controlled indentation, resulting in consistent formatting across all frame contexts while making the code more compact and maintainable. All frame types now use the same base formatting logic without hardcoded spacing assumptions.
- **Fixed embedded frame line formatting**: Corrected missing newlines between embedded frames in CHAP and CTOC display
- **Reasoning**: Fixed the `display_embedded_frame_content()` function in `id3v2_chapter_frame.rs` by changing all `write!(f, ...)` calls to `writeln!(f, ...)` for embedded frame content display. The issue was causing embedded frames to run together on the same line (e.g., "Intel 387 FPU"  [2] APIC - Attached picture") instead of appearing on separate lines. This formatting fix ensures proper visual separation between embedded frames in chapter displays, improving readability and maintaining consistent formatting throughout the tool's output. The fix applies to both CHAP and CTOC frames since they use the same helper function.
- **Embedded frame formatting alignment**: Unified embedded frame display format to match top-level frame formatting structure
- **Reasoning**: Completely refactored `display_embedded_frame_content()` function to format embedded frames using the same structure as top-level frames: "Frame: {ID} ({description}) - Size: {size} bytes" followed by properly indented frame content. Removed the previous numbered format "[{index}] {ID} - {description}" and eliminated the redundant `format_embedded_display()` function. Updated both CHAP and CTOC frame displays to remove numbered headers and rely on the unified formatting function. This provides consistent user experience where embedded frames look identical to top-level frames but with appropriate embedded indentation (10 spaces for frame header, 14 spaces for content), improving readability and maintaining formatting consistency throughout the tool's output. The change also removed dead code warnings by eliminating unused helper functions.
- **Enhanced embedded frame diagnostic display**: Added comprehensive frame header information to embedded frames matching top-level frame diagnostic output
- **Reasoning**: Extended `display_embedded_frame_content()` function to show detailed frame header information for embedded frames, including ID bytes in hexadecimal format, frame ID string, size, and flags. This provides the same level of diagnostic detail for embedded frames as top-level frames, displaying format like "Frame ID bytes = [0x54, 0x49, 0x54, 0x32] ("TIT2"), Size: 45 bytes, Flags: 0x0000" followed by the structured frame content. While embedded frames don't have file offsets (since they exist within parent frames), they now show all other available diagnostic information, maintaining consistency with the tool's comprehensive diagnostic output approach and helping users analyze complex frame structures in podcast chapters and table of contents.
