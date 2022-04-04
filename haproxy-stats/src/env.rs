use core::{fmt, ops::Deref};
use std::{
    collections::HashMap,
    io::{BufRead as _, Cursor, Error as IoError},
};

//
#[derive(Debug, Clone)]
pub struct EnvironmentVariables(pub HashMap<Box<str>, Box<str>>);

impl Deref for EnvironmentVariables {
    type Target = HashMap<Box<str>, Box<str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EnvironmentVariables {
    pub fn from_kv_bytes(
        bytes: impl AsRef<[u8]>,
    ) -> Result<Self, EnvironmentVariablesFromKvBytesError> {
        let bytes = bytes.as_ref();

        let cursor = Cursor::new(bytes);

        let map: HashMap<Box<str>, Box<str>> = cursor
            .lines()
            .map(|x| match x {
                Ok(s) => {
                    let mut split = s.split('=');
                    let k: Box<str> = split.next().unwrap_or_default().into();
                    let v: Box<str> = split.next().unwrap_or_default().into();

                    Ok((k, v))
                }
                Err(err) => Err(EnvironmentVariablesFromKvBytesError::LinesReadFailed(err)),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(map))
    }
}

//
#[derive(Debug)]
pub enum EnvironmentVariablesFromKvBytesError {
    LinesReadFailed(IoError),
}

impl fmt::Display for EnvironmentVariablesFromKvBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EnvironmentVariablesFromKvBytesError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_from_kv_bytes() {
        let bytes = include_bytes!("../tests/files/2_5_5_show_env.txt");

        let vars = EnvironmentVariables::from_kv_bytes(bytes).unwrap();

        assert_eq!(
            vars.get("HAPROXY_VERSION").cloned().unwrap(),
            "2.5.5".into()
        );
    }
}
