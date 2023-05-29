use ccc_lib::types::question::{Question, QuestionType};

fn main() {
    println!("Hello, world!");
    let q = Question::new(2000, QuestionType::Senior, 3, "LazzyCoder".to_string());
    println!("{q:?}");
}
