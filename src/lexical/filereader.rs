use std::fs::File;
use std::io::Read;
// use std::collections::VecDeque;

pub struct FileReader {
    source: String,
    // char_list: VecDeque<char>
}

impl FileReader {

    pub fn new(file_name: &str) -> Self {
        let mut f = File::open(file_name).expect("file not found");
        
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        
        FileReader {
            source: contents
            // char_list: content_clone.chars().collect()
        }
    }

    // pub fn next(&mut self) -> Option<char> {
    //     if self.char_list.len() > 0 {
    //         self.char_list.pop_front()
    //     }
    //     else {
    //         // End of File
    //         Some('\0')
    //     }
    // }

    // pub fn has_next(&self) -> bool {
    //     self.char_list.len() > 0
    // }

    pub fn get_content(&mut self) -> &str {
        &self.source
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_file_reader() {
//         let mut reader = FileReader::new("/Users/kajdreef/Documents/programming/side-projects/rust/rlox/src/lexical/example.lox");

//         assert_eq!(reader.next().unwrap(), 'p');
//         assert_eq!(reader.next().unwrap(), 'r');
//         assert_eq!(reader.next().unwrap(), 'i');
//         assert_eq!(reader.next().unwrap(), 'n');
//         assert_eq!(reader.next().unwrap(), 't');
//     }
// }