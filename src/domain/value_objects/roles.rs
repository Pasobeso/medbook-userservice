use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Patient,
    Doctor,
}

//finding a way to derive string from this enum

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Patient => write!(f, "Patient"),
            Role::Doctor => write!(f, "Doctor"),
        }
    }
}
