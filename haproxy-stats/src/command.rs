use core::{fmt, ops::ControlFlow, str::FromStr};

//
pub(crate) const SEMI_COLON: char = ';';
const BACKSLASH: char = '\\';

//
#[derive(Debug, Clone)]
pub struct Command {
    inner: Box<str>,
}

impl Command {
    pub fn new(command: impl AsRef<str>) -> Result<Self, CommandParseError> {
        let command = command.as_ref();

        let control_flow = command.chars().try_fold(None, |prev, x| {
            if x == SEMI_COLON {
                if prev == Some(BACKSLASH) {
                    ControlFlow::Continue(Some(x))
                } else {
                    ControlFlow::Break(())
                }
            } else {
                ControlFlow::Continue(Some(x))
            }
        });

        match control_flow {
            ControlFlow::Continue(_) => {}
            ControlFlow::Break(_) => return Err(CommandParseError::RequireEscapeSemiColon),
        }

        Ok(Self {
            inner: command.into(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn to_write_bytes(&self) -> Vec<u8> {
        format!("{}\r\n", self.as_str()).as_bytes().to_vec()
    }
}

impl Command {
    pub fn show_info() -> Self {
        Self::new("show info").expect("")
    }

    pub fn show_stat() -> Self {
        Self::new("show stat").expect("")
    }

    pub fn show_env() -> Self {
        Self::new("show env").expect("")
    }
}

//
#[derive(Debug)]
pub enum CommandParseError {
    RequireEscapeSemiColon,
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CommandParseError {}

//
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

//
impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct Commands<'a>(pub &'a [Command]);

impl<'a> Commands<'a> {
    pub fn new(inner: &'a [Command]) -> Self {
        Self(inner)
    }

    fn internal_to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<_>>()
            .join(SEMI_COLON.to_string().as_str())
    }

    pub fn to_write_bytes(&self) -> Vec<u8> {
        format!("{}\r\n", self.internal_to_string())
            .as_bytes()
            .to_vec()
    }
}

//
impl<'a> fmt::Display for Commands<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal_to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new() {
        assert_eq!(Command::show_stat().as_str(), "show stat");

        //
        match Command::new("show stat;") {
            Err(CommandParseError::RequireEscapeSemiColon) => {}
            x => panic!("{:?}", x),
        }
    }
}
