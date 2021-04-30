/*! # Kernel Function Path
 *
 * Lists the system call classes and defines the kernel function path.
 *
 * This is used by the kernel to access the first layer of groups of system
 * call routines
 */

use core::fmt;

use crate::sysc::codes::{
    KernDirFnId,
    KernFileFnId,
    KernIpcChanFnId,
    KernLinkFnId,
    KernMMapFnId,
    KernMutexFnId,
    KernOSEntConfigFnId,
    KernOSEntFnId,
    KernOSGroupFnId,
    KernOSUserFnId,
    KernObjConfigFnId,
    KernObjectFnId,
    KernPathFnId,
    KernProcFnId,
    KernTaskConfigFnId,
    KernTaskFnId,
    KernThreadFnId,
    KernTimeInstFnId,
    KrnIteratorFnId
};
use core::convert::TryFrom;

/** # Kernel Function Path
 *
 * Defines the constructable path to a kernel's system call function.
 *
 * Each variant represents a class (the kernel map's key for the routine
 * array of the object referenced) that contains the related function
 * identifier
 */
#[derive(Debug, Copy, Clone)]
pub enum KernFnPath {
    ObjConfig(KernObjConfigFnId),
    TaskConfig(KernTaskConfigFnId),
    OSEntConfig(KernOSEntConfigFnId),
    Object(KernObjectFnId),
    Task(KernTaskFnId),
    Dir(KernDirFnId),
    File(KernFileFnId),
    IpcChan(KernIpcChanFnId),
    Iterator(KrnIteratorFnId),
    Link(KernLinkFnId),
    MMap(KernMMapFnId),
    Mutex(KernMutexFnId),
    TimeInst(KernTimeInstFnId),
    Path(KernPathFnId),
    OSEntity(KernOSEntFnId),
    OSUser(KernOSUserFnId),
    OSGroup(KernOSGroupFnId),
    Proc(KernProcFnId),
    Thread(KernThreadFnId)
}

impl KernFnPath {
    /** Returns the current function class as `u16`
     */
    pub fn raw_fn_class(&self) -> u16 {
        match self {
            Self::ObjConfig(_) => 0,
            Self::TaskConfig(_) => 1,
            Self::OSEntConfig(_) => 2,
            Self::Object(_) => 3,
            Self::Task(_) => 4,
            Self::Dir(_) => 5,
            Self::File(_) => 6,
            Self::IpcChan(_) => 7,
            Self::Iterator(_) => 8,
            Self::Link(_) => 9,
            Self::MMap(_) => 10,
            Self::Mutex(_) => 11,
            Self::TimeInst(_) => 12,
            Self::Path(_) => 13,
            Self::OSEntity(_) => 14,
            Self::OSUser(_) => 15,
            Self::OSGroup(_) => 16,
            Self::Proc(_) => 17,
            Self::Thread(_) => 18
        }
    }

    /** Returns the current function id as `u16`
     */
    pub fn raw_fn_id(&self) -> u16 {
        match *self {
            Self::ObjConfig(fn_id) => fn_id.into(),
            Self::TaskConfig(fn_id) => fn_id.into(),
            Self::OSEntConfig(fn_id) => fn_id.into(),
            Self::Object(fn_id) => fn_id.into(),
            Self::Task(fn_id) => fn_id.into(),
            Self::Dir(fn_id) => fn_id.into(),
            Self::File(fn_id) => fn_id.into(),
            Self::IpcChan(fn_id) => fn_id.into(),
            Self::Iterator(fn_id) => fn_id.into(),
            Self::Link(fn_id) => fn_id.into(),
            Self::MMap(fn_id) => fn_id.into(),
            Self::Mutex(fn_id) => fn_id.into(),
            Self::TimeInst(fn_id) => fn_id.into(),
            Self::Path(fn_id) => fn_id.into(),
            Self::OSEntity(fn_id) => fn_id.into(),
            Self::OSUser(fn_id) => fn_id.into(),
            Self::OSGroup(fn_id) => fn_id.into(),
            Self::Proc(fn_id) => fn_id.into(),
            Self::Thread(fn_id) => fn_id.into()
        }
    }
}

impl TryFrom<(usize, usize)> for KernFnPath {
    /** The type returned in the event of a conversion error
     */
    type Error = ();

    /** Performs the conversion
     */
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        match value.0 {
            0 => {
                if let Ok(fn_id) = KernObjConfigFnId::try_from(value.1 as u16) {
                    Ok(Self::ObjConfig(fn_id))
                } else {
                    Err(())
                }
            },
            1 => {
                if let Ok(fn_id) = KernTaskConfigFnId::try_from(value.1 as u16) {
                    Ok(Self::TaskConfig(fn_id))
                } else {
                    Err(())
                }
            },
            2 => {
                if let Ok(fn_id) = KernOSEntConfigFnId::try_from(value.1 as u16) {
                    Ok(Self::OSEntConfig(fn_id))
                } else {
                    Err(())
                }
            },
            3 => {
                if let Ok(fn_id) = KernObjectFnId::try_from(value.1 as u16) {
                    Ok(Self::Object(fn_id))
                } else {
                    Err(())
                }
            },
            4 => {
                if let Ok(fn_id) = KernTaskFnId::try_from(value.1 as u16) {
                    Ok(Self::Task(fn_id))
                } else {
                    Err(())
                }
            },
            5 => {
                if let Ok(fn_id) = KernDirFnId::try_from(value.1 as u16) {
                    Ok(Self::Dir(fn_id))
                } else {
                    Err(())
                }
            },
            6 => {
                if let Ok(fn_id) = KernFileFnId::try_from(value.1 as u16) {
                    Ok(Self::File(fn_id))
                } else {
                    Err(())
                }
            },
            7 => {
                if let Ok(fn_id) = KernIpcChanFnId::try_from(value.1 as u16) {
                    Ok(Self::IpcChan(fn_id))
                } else {
                    Err(())
                }
            },
            8 => {
                if let Ok(fn_id) = KrnIteratorFnId::try_from(value.1 as u16) {
                    Ok(Self::Iterator(fn_id))
                } else {
                    Err(())
                }
            },
            9 => {
                if let Ok(fn_id) = KernLinkFnId::try_from(value.1 as u16) {
                    Ok(Self::Link(fn_id))
                } else {
                    Err(())
                }
            },
            10 => {
                if let Ok(fn_id) = KernMMapFnId::try_from(value.1 as u16) {
                    Ok(Self::MMap(fn_id))
                } else {
                    Err(())
                }
            },
            11 => {
                if let Ok(fn_id) = KernMutexFnId::try_from(value.1 as u16) {
                    Ok(Self::Mutex(fn_id))
                } else {
                    Err(())
                }
            },
            12 => {
                if let Ok(fn_id) = KernTimeInstFnId::try_from(value.1 as u16) {
                    Ok(Self::TimeInst(fn_id))
                } else {
                    Err(())
                }
            },
            13 => {
                if let Ok(fn_id) = KernPathFnId::try_from(value.1 as u16) {
                    Ok(Self::Path(fn_id))
                } else {
                    Err(())
                }
            },
            14 => {
                if let Ok(fn_id) = KernOSEntFnId::try_from(value.1 as u16) {
                    Ok(Self::OSEntity(fn_id))
                } else {
                    Err(())
                }
            },
            15 => {
                if let Ok(fn_id) = KernOSUserFnId::try_from(value.1 as u16) {
                    Ok(Self::OSUser(fn_id))
                } else {
                    Err(())
                }
            },
            16 => {
                if let Ok(fn_id) = KernOSGroupFnId::try_from(value.1 as u16) {
                    Ok(Self::OSGroup(fn_id))
                } else {
                    Err(())
                }
            },
            17 => {
                if let Ok(fn_id) = KernProcFnId::try_from(value.1 as u16) {
                    Ok(Self::Proc(fn_id))
                } else {
                    Err(())
                }
            },
            18 => {
                if let Ok(fn_id) = KernThreadFnId::try_from(value.1 as u16) {
                    Ok(Self::Thread(fn_id))
                } else {
                    Err(())
                }
            },
            _ => Err(())
        }
    }
}

impl fmt::Display for KernFnPath {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObjConfig(code) => write!(f, "KernFnPath::ObjConfig({:?})", code),
            Self::TaskConfig(code) => write!(f, "KernFnPath::TaskConfig({:?})", code),
            Self::OSEntConfig(code) => write!(f, "KernFnPath::OSEntConfig({:?})", code),
            Self::Object(code) => write!(f, "KernFnPath::Object({:?})", code),
            Self::Task(code) => write!(f, "KernFnPath::Task({:?})", code),
            Self::Dir(code) => write!(f, "KernFnPath::Dir({:?})", code),
            Self::File(code) => write!(f, "KernFnPath::File({:?})", code),
            Self::IpcChan(code) => write!(f, "KernFnPath::IpcChan({:?})", code),
            Self::Iterator(code) => write!(f, "KernFnPath::Iterator({:?})", code),
            Self::Link(code) => write!(f, "KernFnPath::Link({:?})", code),
            Self::MMap(code) => write!(f, "KernFnPath::MMap({:?})", code),
            Self::Mutex(code) => write!(f, "KernFnPath::Mutex({:?})", code),
            Self::TimeInst(code) => write!(f, "KernFnPath::Time({:?})", code),
            Self::Path(code) => write!(f, "KernFnPath::Path({:?})", code),
            Self::OSEntity(code) => write!(f, "KernFnPath::OSEntity({:?})", code),
            Self::OSUser(code) => write!(f, "KernFnPath::OSUser({:?})", code),
            Self::OSGroup(code) => write!(f, "KernFnPath::OSGroup({:?})", code),
            Self::Proc(code) => write!(f, "KernFnPath::Proc({:?})", code),
            Self::Thread(code) => write!(f, "KernFnPath::Thread({:?})", code)
        }
    }
}
