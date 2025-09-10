use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    Patient,
    Doctor,
}

//finding a way to derive string from this enum

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Roles::Patient => write!(f, "Patient"),
            Roles::Doctor => write!(f, "Doctor"),
        }
    }
}
