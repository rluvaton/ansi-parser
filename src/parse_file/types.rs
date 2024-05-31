use crate::files::file_reader::FileReaderOptions;
use crate::parse_ansi_text::parse_options::ParseOptions;

pub struct ReadAnsiFileOptions {
    pub file_options: FileReaderOptions,
    pub parse_options: ParseOptions,
}
