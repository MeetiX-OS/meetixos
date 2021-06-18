/*! `OSEntity` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available obj types represented by an `OSEntityId`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum OsEntityType {
    /**
     * Default value
     */
    Unknown,

    /**
     * Identifies an `OSUser` ent
     */
    User,

    /**
     * Identifies an `OSGroup` ent
     */
    Group
}

impl Default for OsEntityType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for OsEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::User => write!(f, "User"),
            Self::Group => write!(f, "Group")
        }
    }
}