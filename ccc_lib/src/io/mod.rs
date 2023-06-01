pub mod debug;
pub mod input;
pub mod log;
pub mod provider;

pub trait IoWriter {
    fn write_line(&self, content: &str) -> Option<bool>;
}

pub trait IoReader {
    fn read_line(&self) -> Option<String>;
}
