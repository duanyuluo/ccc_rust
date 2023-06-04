pub mod debug;
pub mod input;
pub mod log;
pub mod provider;
// pub mod tbl;

pub trait IoWriter {
    fn write_line(&mut self, content: &str) -> Option<bool>;
}

pub trait IoReader {
    fn read_line(&mut self) -> Option<String>;
}
