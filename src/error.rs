use std::{error::Error, fmt::Display, io, string::FromUtf8Error};

#[derive(Debug)]
pub enum MyUuidError {
    OsRngUnavailable(io::Error),
    FromUtf8Error(FromUtf8Error),
}

impl Error for MyUuidError {}

impl Display for MyUuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyUuidError::OsRngUnavailable(error) => {
                write!(f, "OSの乱数生成器が利用できません: {error}")
            }
            MyUuidError::FromUtf8Error(error) => write!(f, "UTF8への変換に失敗しました: {error}"),
        }
    }
}

impl From<io::Error> for MyUuidError {
    fn from(value: io::Error) -> Self {
        Self::OsRngUnavailable(value)
    }
}

impl From<FromUtf8Error> for MyUuidError {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}
