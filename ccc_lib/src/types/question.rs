/// Question
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum QuestionType {
    Junior,
    Senior,
}

impl Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionType::Junior => write!(f, "Junior"),
            QuestionType::Senior => write!(f, "Senior"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    year: u16,
    level: QuestionType,
    num: u8,
    title: String,
}

impl Question {
    pub fn new(year: u16, level: QuestionType, num: u8, title: String) -> Question {
        Question {
            year,
            level,
            num,
            title,
        }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Q[{}-{}-#{}]: {}",
            self.year, self.level, self.num, self.title
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_default_question() -> Question {
        Question::new(2015, QuestionType::Senior, 3, "HelloWorld".to_string())
    }

    #[test]
    fn question_new() {
        let q = new_default_question();
        assert_eq!(q.year, 2015);
        assert_eq!(q.level, QuestionType::Senior);
        assert_eq!(q.num, 3);
        assert_eq!(q.title, "HelloWorld");
    }

    #[test]
    fn question_display() {
        let q = new_default_question();
        assert_eq!(format!("{q}"), "Q[2015-Senior-#3]: HelloWorld");
    }

    #[test]
    fn question_type_display() {
        let qt = QuestionType::Junior;
        assert_eq!(format!("{qt}"), "Junior");
        let qt = QuestionType::Senior;
        assert_eq!(format!("{qt}"), "Senior");
    }

    #[test]
    fn question_type_in_question() {
        let q = new_default_question();
        match q.level {
            QuestionType::Junior => assert!(false),
            _ => assert!(true),
        }
    }
}
