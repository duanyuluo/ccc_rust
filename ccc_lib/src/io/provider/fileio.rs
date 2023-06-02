use crate::io::{IoReader, IoWriter};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone)]
pub struct FileIoReadProvider {
    pub file_path: String,
    pub lines: Vec<String>,
}

impl IoReader for FileIoReadProvider {
    fn read_line(&mut self) -> Option<String> {
        self.lines.pop()
    }
}

impl FileIoReadProvider {
    /// open_for_read:
    /// open a existing file, then read lines to FileIoReadProvider.lines[]
    pub fn open_for_read(file_path: &str) -> Option<Box<dyn IoReader>> {
        let file_path = String::from(file_path);
        let mut read_provider = FileIoReadProvider {
            file_path,
            lines: vec![],
        };
        let r = File::open(read_provider.file_path.clone());
        match r {
            Ok(handle) => {
                let buffer = BufReader::new(handle);
                for line in buffer.lines() {
                    read_provider
                        .lines
                        .push(line.unwrap().trim_end().to_string());
                }
                read_provider.lines.reverse();
                Some(Box::new(read_provider))
            }
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct FileIoWriteProvider {
    file_path: String,
    output_handle: File,
}

impl IoWriter for FileIoWriteProvider {
    fn write_line(&mut self, content: &str) -> Option<bool> {
        let s = format!("{}\n", content);
        let buf = s.as_bytes();
        let r = self.output_handle.write_all(buf);
        match r {
            Ok(_) => {
                self.output_handle.flush().unwrap();
                return Some(true);
            }
            Err(_) => {
                println!("file '{}' not exist.", self.file_path);
                return None;
            }
        };
    }
}

impl FileIoWriteProvider {
    pub fn open_for_write(file_path: &str) -> Option<Box<dyn IoWriter>> {
        let write_provider = FileIoWriteProvider {
            file_path: String::from(file_path),
            output_handle: File::create(file_path).unwrap(),
        };
        Some(Box::new(write_provider))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_from_file() {
        let reader_pack =
            FileIoReadProvider::open_for_read("/Users/lzcoder/repos/ccc-rust/io_files/test.txt");
        let mut reader = reader_pack.unwrap();
        let ln = reader.read_line().unwrap();
        assert_eq!(ln, "This is a test file1.");
        let ln = reader.read_line().unwrap();
        assert_eq!(ln, "This is a test file2.");
    }

    #[test]
    fn write_to_file() {
        let writer =
            FileIoWriteProvider::open_for_write("/Users/lzcoder/repos/ccc-rust/io_files/write.txt");
        let mut handle = writer.unwrap();
        handle.write_line("This is a test file.");
        handle.write_line("This is another test file.");
        assert!(true);
    }
}
