pub mod enums;
pub mod output;
pub mod parsers;
mod complete_parsers;
mod parsers_only_text_and_graphics_manual;
pub mod parsers_only_text_and_graphics_manual_simd;
mod parsers_only_text_and_graphics;


// Make it public to consumers of the library, aka. external API
pub use enums::AnsiSequence;
pub use output::{Output, Text};
pub use parsers::parse_escape;
pub use complete_parsers::parse_escape as parse_escape_complete;
pub use parsers_only_text_and_graphics_manual::parse_escape as parse_escape_only_text_and_graphics_manual;
pub use parsers_only_text_and_graphics_manual_simd::parse_escape as parse_escape_only_text_and_graphics_manual_simd;
pub use parsers_only_text_and_graphics::parse_escape as parse_escape_only_text_and_graphics;
