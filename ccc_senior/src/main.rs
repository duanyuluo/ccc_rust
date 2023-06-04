use ccc_lib::types::question::*;
use std::env;

mod ccc03s3;

fn main() {
    let args: Vec<String> = env::args().collect();
    for (idx, arg) in args.iter().enumerate() {
        println!("{idx} => {arg}");
    }

    let q = ccc03s3::get_question();
    display_question_summary(q);
    ccc03s3::run_solver();
}

fn display_question_summary(q: Question) {
    println!("Title:  {}", q.title);
    println!("Year:   {}", q.year);
    println!("Number: {}{}", q.level, q.num);
    println!("Description:");
    println!("{}", q.description);
}

#[cfg(test)]
mod test {
    use super::*;
    // use ccc_lib::types::question::*;

    #[test]
    fn main_import_ccc_lib() {
        let q = Question::new(2000, QuestionType::default(), 3, "HelloWorld".to_string());
        assert_eq!(q.level, QuestionType::Senior);
    }
}
