/// Question module: question meta datas
/// define struct Question, QuestionType
use std::fmt::Display;

/// QuestionType:
/// Question's level in CCC: Junior, Senior(default)
#[derive(Debug, Clone, Default, PartialEq)]
pub enum QuestionType {
    Junior,
    #[default]
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

/// # Question:
/// ## Question meta data:
///   - year: 4-digits number
///   - level: enum QuestionType
///   - num: index in the exam
///   - title: question's title
///   - description: question's description
/// ## Samples
///   Question::new(2015, QuestionType::default(), 3, "HelloWorld".to_string())
/// ## References
///   - [CCC](https://github.com/lzcoder/ccc)
#[derive(Debug, Clone)]
pub struct Question {
    pub year: u16,
    pub level: QuestionType,
    pub num: u8,
    pub title: String,
    description: String,
}

impl Question {
    /// Question::new() create a Question object.
    pub fn new(year: u16, level: QuestionType, num: u8, title: String) -> Question {
        Question {
            year,
            level,
            num,
            title,
            description: "".to_string(),
        }
    }

    /// Question::define() append question description to Question::description.
    pub fn define(&mut self, description: &str) {
        self.description.push_str(description);
    }
}

/// ## Display
/// Customize Display for Question
impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.description.is_empty() {
            write!(
                f,
                "Q[{}-{}-#{}]: {}",
                self.year, self.level, self.num, self.title
            )
        } else {
            write!(
                f,
                "Q[{}-{}-#{}]: {}\n{}",
                self.year, self.level, self.num, self.title, self.description
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_default_question() -> Question {
        Question::new(2015, QuestionType::default(), 3, "HelloWorld".to_string())
    }

    #[test]
    fn question_new_default() {
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
    fn question_define() {
        let mut q = new_default_question();
        q.define("This world is so beautiful.");
        assert_eq!(q.description, "This world is so beautiful.");
        assert_eq!(
            format!("{q}"),
            "Q[2015-Senior-#3]: HelloWorld\nThis world is so beautiful."
        );
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
