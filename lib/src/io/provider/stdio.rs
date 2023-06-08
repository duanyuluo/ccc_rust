use std::io::{stdin, stdout, BufRead, Write};

use crate::io::{IoReader, IoWriter};

pub struct StdIoProvider {
    multi_threads: bool,
}

impl IoReader for StdIoProvider {
    fn read_line(&mut self) -> Option<String> {
        let mut input_string = String::new();
        if self.multi_threads {
            stdin()
                .lock()
                .read_line(&mut input_string)
                .expect("err: input please...");
        } else {
            stdin()
                .read_line(&mut input_string)
                .expect("error: input please...");
        }
        Some(input_string)
    }
}

impl IoWriter for StdIoProvider {
    fn write_line(&mut self, content: &str) -> Option<bool> {
        if self.multi_threads {
            stdout()
                .lock()
                .write_fmt(format_args!("{}\n", content))
                .expect("err: output please...");
        } else {
            stdout()
                .write_fmt(format_args!("{}\n", content))
                .expect("err: output please...");
        }
        Some(true)
    }
}

impl StdIoProvider {
    pub fn new_reader(multi_threads: bool) -> Option<Box<dyn IoReader>> {
        let std_io_provider = StdIoProvider { multi_threads };
        Some(Box::new(std_io_provider))
    }
    pub fn new_writer(multi_threads: bool) -> Option<Box<dyn IoWriter>> {
        let std_io_provider = StdIoProvider { multi_threads };
        Some(Box::new(std_io_provider))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_io_reader_provider_withlock() {
        let std_io_reader_pack = StdIoProvider::new_reader(true);
        assert!(std_io_reader_pack.is_some());
        // let std_io_reader = std_io_reader_pack.unwrap();
        // let result = std_io_reader.read_line();
        // assert!(result.is_some());
        // assert_eq!("Ok", result.unwrap());
    }

    #[test]
    fn std_io_writer_privider_withlock() {
        let std_io_writer_pack = StdIoProvider::new_writer(true);
        assert!(std_io_writer_pack.is_some());
        let mut std_io_writer = std_io_writer_pack.unwrap();
        let result = std_io_writer.write_line("Hello World");
        assert!(result.is_some());
    }
}
