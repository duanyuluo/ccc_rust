use crate::io::provider::fileio::FileIoReadProvider;
use crate::io::IoReader;

const CCC_INPUT_DIR: &str = "/Users/lzcoder/repos/ccc-rust/io_files/";

pub struct TcFileInput {
    pub file_handle: Box<dyn IoReader>,
}

/// CCC Testcase File Input
/// for replacement of user input
impl TcFileInput {
    /// Load CCC sample input file
    ///   file_name: sample file in CCC_INPUT_DIR dir. these patterns are cccYYSN[_casename].tc
    pub fn load_testfile(file_name: &str) -> TcFileInput {
        let file_path = CCC_INPUT_DIR.to_owned() + file_name;
        let provider = FileIoReadProvider::open_for_read(&file_path);
        TcFileInput {
            file_handle: provider.unwrap(),
        }
    }

    /// Read int input in a single line.
    pub fn read_int(&mut self) -> i32 {
        let r = self.file_handle.read_line().unwrap();
        let parse_r = r.parse::<i32>();
        match parse_r {
            Ok(n) => return n,
            Err(e) => {
                println!("{}", e);
                return 0;
            }
        };
    }

    /// Read String input in a single line.
    pub fn read_str(&mut self) -> String {
        self.file_handle.read_line().unwrap()
    }

    /// Read vec that input in a single line.
    /// sep_by_space: words seperated by " "(space) or char in String.
    pub fn read_vec(&mut self, sep_by_space: bool) -> Vec<String> {
        let ln = self.file_handle.read_line().unwrap();
        if sep_by_space {
            ln.split_whitespace().map(|s| String::from(s)).collect()
        } else {
            let mut chunks = Vec::new();
            for c in ln.chars() {
                let mut cs = c.to_string();
                chunks.push(std::mem::take(&mut cs));
            }
            chunks
        }
    }

    /// Read and type casting T to vec that input in a single line.
    /// T: std::str::FromStr + Default
    pub fn read_vec_t<T: std::str::FromStr + Default>(&mut self) -> Vec<T> {
        self.read_vec(true)
            .iter()
            .map(|s| s.parse::<T>().unwrap_or_default())
            .collect()
    }
}

/// Read and type casting $type(2nd arg) to tuple that input in a single line.
/// eg. let (a, b) = read_tuple_t(io_reader, i32);
macro_rules! read_tuple_t {
    ($obj:ident, $type:ty) => {
        $obj.read_vec_t::<$type>()
            .into_iter()
            .collect_tuple()
            .unwrap()
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn read_file_to_int_and_vec_and_tuple() {
        let mut input = TcFileInput::load_testfile("test_input.tc");
        assert_eq!(input.read_int(), 105);
        assert_eq!(input.read_int(), 14);
        assert_eq!(input.read_int(), 16);
        assert_eq!(
            input.read_vec(false),
            vec!["I", "I", "I", "I", "I", "I", "I", "I", "I", "I", "I", "I", "I", "I", "I", "I"]
        );
        assert_eq!(
            input.read_vec(false),
            vec!["I", ".", ".", ".", ".", ".", ".", "I", ".", ".", ".", ".", ".", ".", ".", "I"]
        );
        assert_eq!(input.read_vec(true), vec!["a", "b", "c", "xyz"]);
        assert_eq!(input.read_vec_t::<i16>(), vec![1, 2, 3]);
        let (a, b, c) = read_tuple_t!(input, i16);
        assert_eq!((a, b, c), (1, 2, 3));
    }
}
