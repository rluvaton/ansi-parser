use std::fs::File as File;
use std::path::PathBuf;

use get_chunk::iterator::FileIter;

pub fn create_file_iterator(input_file_path: PathBuf) -> Box<dyn Iterator<Item = String>> {
    let input_file = File::open(input_file_path).expect("opening input file path failed");

    let file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");
    let file_string_iterator = file_iter.into_iter().map(|item| {
        String::from_utf8_lossy(item.expect("Failed to get file chunk").as_ref()).to_string()
    });

    return Box::new(file_string_iterator);
}

pub fn create_file_iterator_from_to_locations(
    input_file_path: PathBuf,
    from_line: Option<usize>,
    to_line: Option<usize>,
) -> Box<dyn Iterator<Item = String>> {
    if from_line.is_none() && to_line.is_none() {
        return create_file_iterator(input_file_path);
    }

    let input_file = File::open(input_file_path).expect("opening input file path failed");

    let mut file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");
    if from_line.is_some() {
        file_iter = file_iter
            .set_start_position_bytes(from_line.unwrap())
            .expect("Failed to set start position");
    }

    if to_line.is_none() {
        let file_string_iterator = file_iter.into_iter().map(|item| { 
            String::from_utf8_lossy(item.expect("Failed to get file chunk").as_ref()).to_string()
        });

        return Box::new(file_string_iterator);
    }

    let end = to_line.unwrap() - from_line.unwrap();

    let mut current = 0;
    let mut should_trim = false;
    let mut trim_size: usize = 0;
    let mut item_index = 0;
    let mut item_map_index = 0;

    let file_string_iterator = file_iter
        .into_iter()
        // Take until reaching the end
        .take_while(move |item| {
            item_index += 1;
            let len = item.as_ref().expect("Failed to get file chunk").len();

            if current + len > end && current < end {
                should_trim = true;
                trim_size = end - current;

                current += len;
                return true;
            }
            current += len;
            return current < end;
        })
        // Converting to string while also trimming the last item if needed
        .map(move |item| {
            // Making sure trimming is done to the correct item
            item_map_index += 1;
            if should_trim && item_index == item_map_index {
                return String::from_utf8_lossy(item.expect("Failed to get file chunk")[..trim_size].as_ref()).to_string();
            }

            return String::from_utf8_lossy(item.expect("Failed to get file chunk").as_ref()).to_string();
        });

    return Box::new(file_string_iterator);
}
