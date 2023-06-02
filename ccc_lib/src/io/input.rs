use crate::io::provider::fileio::FileIoReadProvider;
use crate::io::IoReader;

const CCC_INPUT_DIR: &str = "/Users/lzcoder/repos/ccc-rust/io_files/";

pub struct TcFileInput {
    pub file_handle: Box<dyn IoReader>,
}

impl TcFileInput {
    pub fn load_testfile(file_name: &str) -> TcFileInput {
        let file_path = CCC_INPUT_DIR.to_owned() + file_name;
        let provider = FileIoReadProvider::open_for_read(&file_path);
        TcFileInput {
            file_handle: provider.unwrap(),
        }
    }

    pub fn read_int(&mut self) -> i32 {
        let r = self.file_handle.read_line().unwrap();
        println!("{}", r);
        let parse_r = r.parse::<i32>();
        match parse_r {
            Ok(n) => return n,
            Err(e) => {
                println!("{}", e);
                return 0;
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_file_to_int() {
        let mut input = TcFileInput::load_testfile("ccc03s3.tc");
        assert_eq!(input.read_int(), 105);
        assert_eq!(input.read_int(), 14);
        assert_eq!(input.read_int(), 16);
    }
}
