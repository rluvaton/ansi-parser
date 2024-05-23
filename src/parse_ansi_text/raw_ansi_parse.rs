pub mod enums;
pub mod parsers;

// Make it public to consumers of the library, aka. external API
pub use enums::AnsiSequence;
pub use parsers::parse_escape;

