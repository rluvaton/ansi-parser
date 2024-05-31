pub mod enums;
pub mod output;
pub mod parsers;

// Make it public to consumers of the library, aka. external API
pub use enums::AnsiSequence;
pub use output::{Output, Text};
pub use parsers::parse_escape;
