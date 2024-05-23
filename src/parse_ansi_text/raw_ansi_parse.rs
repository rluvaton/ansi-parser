pub mod enums;
pub mod parsers;
pub mod output;

// Make it public to consumers of the library, aka. external API
pub use enums::AnsiSequence;
pub use parsers::parse_escape;
pub use output::{Output, Text};

