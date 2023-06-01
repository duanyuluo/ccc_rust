use ccc_lib::types::question::{Question, QuestionType};
use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    for (idx, arg) in args.iter().enumerate() {
        println!("{idx} => {arg}");
    }

    let q = Question::new(2000, QuestionType::default(), 3, "LazzyCoder".to_string());
    println!("{q:?}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main_import_ccc_lib() {
        let q = Question::new(2000, QuestionType::default(), 3, "HelloWorld".to_string());
        assert_eq!(q.level, QuestionType::Senior);
    }
}
