use const_typed_builder::Builder;
use get_chunk::iterator::FileIter;
use get_chunk::ChunkSize;
use std::fs::File;

pub struct FileReader {
    file_iter: FileIter<File>,
    to_bytes: Option<usize>,
    current_position: usize,
}

impl Iterator for FileReader {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_bytes.is_some() && self.current_position >= self.to_bytes.unwrap() {
            return None;
        }

        let chunk = self
            .file_iter
            .next()
            .map(|item| item.expect("Failed to get file chunk"));

        if chunk.is_none() {
            return None;
        }

        let chunk = chunk.unwrap();

        if let Some(to_bytes) = self.to_bytes {
            if self.current_position + chunk.len() > to_bytes {
                self.current_position = to_bytes;
                return Some(chunk[..self.current_position - to_bytes].to_vec());
            }
        }

        self.current_position += chunk.len();

        return Some(chunk);
    }
}

impl FileReader {
    pub fn new(options: FileReaderOptions) -> FileReader {
        let file = File::open(options.file_path).expect("opening input file path failed");

        let mut file_iter = FileIter::try_from(file).expect("create input file iterator failed");

        if let Some(chunk_size) = options.chunk_size_in_bytes {
            file_iter = file_iter.set_mode(ChunkSize::Bytes(chunk_size));
        }

        if let Some(from_bytes) = options.from_bytes {
            file_iter = file_iter
                .set_start_position_bytes(from_bytes)
                .expect("Failed to set start position");
        }

        FileReader {
            file_iter,
            to_bytes: options.to_bytes,
            current_position: options.from_bytes.unwrap_or(0),
        }
    }
}

#[derive(Debug, Builder)]
pub struct FileReaderOptions {
    pub chunk_size_in_bytes: Option<usize>,
    pub file_path: String,
    pub from_bytes: Option<usize>,
    pub to_bytes: Option<usize>,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use super::*;

    fn get_tmp_file_path() -> String {
        return NamedTempFile::new()
            .expect("create temp file")
            .into_temp_path()
            .to_str()
            .expect("convert to string")
            .to_string();
    }

    fn create_tmp_file(input: &str) -> String {
        let file_path = get_tmp_file_path();

        std::fs::write(file_path.clone(), input).expect("Failed to write to file");

        return file_path;
    }
    
    fn read_all_file_from_iterator(mut reader: FileReader) -> Vec<u8> {
        let mut result = Vec::new();
        while let Some(chunk) = reader.next() {
            result.extend(chunk);
        }

        return result;
    }

    #[test]
    fn test_read_file_contain_the_whole_file() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, expected_file_string.as_bytes());
    }
    
    #[test]
    fn test_read_file_contain_the_whole_file_with_chunks() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .chunk_size_in_bytes(Some(1))
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, expected_file_string.as_bytes());
    }

    #[test]
    fn test_read_file_start_from_the_starting_position() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .from_bytes(expected_file_string.find("World!"))
            .build();
        let file_reader = FileReader::new(options);
        
        let result = read_all_file_from_iterator(file_reader);
        
        assert_eq!(result, "World!".as_bytes());
    }

    #[test]
    fn test_read_file_ends_in_ending_position() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .to_bytes(expected_file_string.find(","))
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, "Hello".as_bytes());
    }

    #[test]
    fn test_read_file_start_and_ends_in_ending_position() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .from_bytes(expected_file_string.find("World"))
            .to_bytes(expected_file_string.find("!"))
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, "World".as_bytes());
    }

    #[test]
    fn test_read_file_start_from_the_starting_position_with_chunks() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .from_bytes(expected_file_string.find("World"))
            .chunk_size_in_bytes(Some(1))
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, "World!".as_bytes());
    }

    #[test]
    fn test_read_file_ends_in_ending_position_with_chunks() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .to_bytes(expected_file_string.find(" "))
            .chunk_size_in_bytes(Some(1))
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, "Hello,".as_bytes());
    }

    #[test]
    fn test_read_file_start_and_ends_in_ending_position_with_chunks() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .from_bytes(expected_file_string.find("World"))
            .to_bytes(expected_file_string.find("!"))
            .chunk_size_in_bytes(Some(1))
            .build();
        let file_reader = FileReader::new(options);

        let result = read_all_file_from_iterator(file_reader);

        assert_eq!(result, "World".as_bytes());
    }

    #[test]
    fn test_get_requested_chunk_size_single_byte() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .chunk_size_in_bytes(Some(1))
            .build();
        let mut file_reader = FileReader::new(options);

        let mut result = Vec::new();
        while let Some(chunk) = file_reader.next() {
            assert_eq!(chunk.len(), 1);
            result.extend(chunk);
        }

        assert_eq!(result, expected_file_string.as_bytes());
    }
    
    #[test]
    fn test_get_requested_chunk_size_smaller_than_file_size() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .chunk_size_in_bytes(Some(3))
            .build();
        let mut file_reader = FileReader::new(options);

        let mut result = Vec::new();
        while let Some(chunk) = file_reader.next() {
            result.push(chunk);
        }
        
        let sizes: Vec<usize> = result.clone().into_iter().map(|chunk| chunk.len()).collect();
        let expected_sizes = vec![
            3, // Hel
            3, // lo,
            3, //  Wo (space in the beginning)
            3, // rld,
            1, // !
        ];
        
        assert_eq!(sizes, expected_sizes);

        assert_eq!(result.into_iter().flatten().collect::<Vec<u8>>(), expected_file_string.as_bytes());
    }

    #[test]
    fn test_get_requested_chunk_size_exactly_file_size() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .chunk_size_in_bytes(Some(expected_file_string.len()))
            .build();
        let mut file_reader = FileReader::new(options);

        let mut result = Vec::new();
        while let Some(chunk) = file_reader.next() {
            result.push(chunk);
        }

        assert_eq!(result, vec![expected_file_string.as_bytes()]);
    }

    #[test]
    fn test_get_requested_chunk_size_greater_than_file_size() {
        let expected_file_string = "Hello, World!";
        let tmp_file_path = create_tmp_file(expected_file_string);

        let options = FileReaderOptions::builder()
            //
            .file_path(tmp_file_path.clone())
            .chunk_size_in_bytes(Some(expected_file_string.len() + 10))
            .build();
        let mut file_reader = FileReader::new(options);

        let mut result = Vec::new();
        while let Some(chunk) = file_reader.next() {
            result.push(chunk);
        }

        assert_eq!(result, vec![expected_file_string.as_bytes()]);
    }
}
