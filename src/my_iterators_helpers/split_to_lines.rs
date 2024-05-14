use std::iter::Iterator;

pub struct SplitToLines<IteratorType> {
    iter: IteratorType,
    pending_chunk: String,
}

/**
* Same implementation as the following in TypeScript:
* ```ts
* function* splitToLines(chunks: Iterator<string>): Iterator<string> {
*     let line = '';
*     for (let chunk of chunks) {
*         while(chunk.includes('\n')) {
*             const i = chunk.indexOf('\n');
*             line += chunk.slice(0, i);
*             yield line;
*             line = '';
*             chunk = chunk.slice(i + 1);
*         }
* 
*         line += chunk;
*     }
* 
*     if (line) {
*         yield line;
*     }
* }
* ```
*/
impl<IteratorType> Iterator for SplitToLines<IteratorType>
    where
        IteratorType: Iterator<Item = String>,
{
    // Output item
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If not empty string than add to line and return line
        if self.pending_chunk.contains("\n") {
            // If we have new line in pending chunk than merge with current line and return
            let i = self.pending_chunk.find("\n").unwrap();

            // Get text until the first new line
            let line = self.pending_chunk[..i].to_string();

            // Set the rest of the text as pending chunk
            self.pending_chunk = self.pending_chunk[(i + 1)..].to_string();

            // Return the line
            return Some(line.to_string());
        }

        // If here than pending chunk does not have new line in it and can be the beginning of the next line
        
        while let Some(chunk) = self.iter.next() {
            // until reaching new line can keep adding to the line
            if !chunk.contains("\n") {
                // Concat pending chunk with current chunk
                // TODO - find a better way
                self.pending_chunk += &chunk;
                continue;
            }
            
            // If here, then we have new line in the chunk
            
            // Get the index of the new line character
            let i = chunk.find("\n").unwrap();
            
            // get the part of the chunk until the new line character
            let part_of_chunk_until_new_line = &chunk[..i];
            
            let line = self.pending_chunk.to_string() + &part_of_chunk_until_new_line;
            
            // Get the rest of the chunk after the new line character
            let rest_of_chunk = &chunk[(i + 1)..];
            
            self.pending_chunk = rest_of_chunk.to_string();
            
            return Some(line.to_string());
        }
        
        // If here, then we have reached the end of the input iterator
        if(!self.pending_chunk.is_empty()) {
            let line = self.pending_chunk.to_string();
            
            // So next call will go to return none
            self.pending_chunk = "".to_string();
            
            return Some(line);
        }
        
        return None;
    }
}

impl<IteratorType> SplitToLines<IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self { iter, pending_chunk: "".to_string() }
    }
}

pub trait SplitToLinesByIterator: Iterator<Item = String> + Sized {
    fn to_lines(self) -> SplitToLines<Self> {
        SplitToLines::new(self)
    }
}

impl<IteratorType: Iterator<Item = String>> SplitToLinesByIterator for IteratorType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_to_lines_should_work() {
        let input = "";

        let chunks = vec!["abc".to_string(), "d\nef\ng".to_string(), "hij".to_string()];
        let lines: Vec<String> = chunks.into_iter().to_lines().collect();

        let expected = vec![
            "abcd".to_string(),
            "ef".to_string(),
            "ghij".to_string(),
        ];
        assert_eq!(lines, expected);
    }
}
