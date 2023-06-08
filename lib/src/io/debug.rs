/// debug tools for ccc
///
use core::fmt::Display;

/// DebugLevel enum all informations' level types.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum DebugLevel {
    Info = 0,
    #[default]
    Warn,
    Error,
    Answer,
}

impl Display for DebugLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugLevel::Info => write!(f, "INFO"),
            DebugLevel::Warn => write!(f, "WARN"),
            DebugLevel::Error => write!(f, "ERRO"),
            DebugLevel::Answer => write!(f, "ANSW"),
        }
    }
}

impl DebugLevel {
    pub fn can_trigger(&self, current_level: &DebugLevel) -> bool {
        if (*self as u8) > *current_level as u8 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug_level_default() {
        let d = DebugLevel::default();
        assert_eq!(DebugLevel::Warn, d);
    }

    #[test]
    fn debug_level_display() {
        assert_eq!(DebugLevel::Info.to_string(), "INFO");
        assert_eq!(DebugLevel::Warn.to_string(), "WARN");
        assert_eq!(DebugLevel::Error.to_string(), "ERRO");
        assert_eq!(DebugLevel::Answer.to_string(), "ANSW");
    }

    #[test]
    fn debug_level_cmp() {
        let lvl = DebugLevel::Warn;
        assert_eq!(true, lvl.can_trigger(&DebugLevel::Error));
        assert_eq!(false, lvl.can_trigger(&DebugLevel::Info));
    }
}
