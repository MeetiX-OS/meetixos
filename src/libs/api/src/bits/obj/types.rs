/*! # `Object` Types
 *
 * Implements the variants that identifies the various [`ObjId`]
 * implementations
 *
 * [`ObjId`]: crate::objs::object::ObjId
 */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `Object` Types
 *
 * Lists the available object types represented by an [`ObjId`]
 *
 * [`ObjId`]: crate::objs::object::ObjId
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjType {
    /** No real uses, used as default value
     */
    Unknown,

    /** Identifies a [`File`] object
     *
     * [`File`]: crate::objs::impls::file::File
     */
    File,

    /** Identifies a [`Dir`] object
     *
     * [`Dir`]: crate::objs::impls::dir::Dir
     */
    Dir,

    /** Identifies a [`Link`] object
     *
     * [`Link`]: crate::objs::impls::link::Link
     */
    Link,

    /** Identifies a [`MMap`] object
     *
     * [`MMap`]: crate::objs::impls::mmap::MMap
     */
    MMap,

    /** Identifies an [`IpcChan`] object
     *
     * [`IpcChan`]: crate::objs::impls::mmap::MMap
     */
    IpcChan,

    /** Identifies an [`OsRawMutex`] object
     *
     * [`OsRawMutex`]: crate::objs::impls::mutex::OsRawMutex
     */
    OsRawMutex,

    /** Identifies an [`KrnIterator`] object
     *
     * [`KrnIterator`]: crate::objs::impls::iter::KrnIterator
     */
    KrnIterator
}

impl Default for ObjType {
    /** Returns the "default value" for a type
     */
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for ObjType {
    /** Formats the value using the given formatter
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::File => write!(f, "File"),
            Self::Dir => write!(f, "Dir"),
            Self::Link => write!(f, "Link"),
            Self::MMap => write!(f, "MMap"),
            Self::IpcChan => write!(f, "IpcChan"),
            Self::OsRawMutex => write!(f, "OsRawMutex"),
            Self::KrnIterator => write!(f, "KrnIterator")
        }
    }
}