use crate::files::file_reader::FileReaderOptions;
use crate::parse_ansi_text::parse_options::ParseOptions;

pub struct ReadAnsiFileOptions {
    pub(crate) file_options: FileReaderOptions,
    pub(crate) parse_options: ParseOptions,
}
