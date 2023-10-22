use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum StatsError {
    FileDoesNotExist(String),
    InvalidRecord(String),
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::FileDoesNotExist(err) => {
                write!(f, "FileDoesNotExist:\n\t{}", err)
            }
            Self::InvalidRecord(err) => {
                write!(f, "InvalidRecord:\n\t{}", err)
            }
        }
    }
}

impl error::Error for StatsError {}
