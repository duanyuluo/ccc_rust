/// debug tools for ccc
///
use core::fmt::Display;

/// DebugLevel enum all informations' level types.
#[derive(Debug)]
pub enum DebugLevel {
    Info,
    Warn,
    Error,
}

impl Display for DebugLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugLevel::Info => write!(f, "INFO"),
            DebugLevel::Warn => write!(f, "WARN"),
            DebugLevel::Error => write!(f, "ERRO"),
        }
    }
}

#[test]
fn test_debug_level() {
    assert_eq!(DebugLevel::Info.to_string(), "INFO");
    assert_eq!(DebugLevel::Warn.to_string(), "WARN");
    assert_eq!(DebugLevel::Error.to_string(), "ERRO");
}
