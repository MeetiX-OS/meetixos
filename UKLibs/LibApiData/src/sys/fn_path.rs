/*! Kernel function call paths */

use core::{
    convert::TryFrom,
    fmt
};

use crate::sys::codes::{
    KernDeviceFnId,
    KernDirFnId,
    KernFileFnId,
    KernIpcChanFnId,
    KernLinkFnId,
    KernMMapFnId,
    KernMutexFnId,
    KernObjConfigFnId,
    KernObjectFnId,
    KernOsEntConfigFnId,
    KernOsEntFnId,
    KernOsGroupFnId,
    KernOsUserFnId,
    KernPathFnId,
    KernProcFnId,
    KernTaskConfigFnId,
    KernTaskFnId,
    KernThreadFnId,
    KernTimeInstFnId,
    KrnIteratorFnId
};

/**
 * Lists the callable Kernel function paths.
 *
 * Each variant represent a Kernel call class, which is the primary key of
 * the Kernel's routines table, and each class contains the specific codes
 * for the call class, which is the secondary key of the Kernel's routines
 * table
 */
#[derive(Debug, Copy, Clone)]
pub enum KernFnPath {
    ObjConfig(KernObjConfigFnId),
    TaskConfig(KernTaskConfigFnId),
    OsEntConfig(KernOsEntConfigFnId),
    Object(KernObjectFnId),
    Task(KernTaskFnId),
    Device(KernDeviceFnId),
    Dir(KernDirFnId),
    File(KernFileFnId),
    IpcChan(KernIpcChanFnId),
    Iterator(KrnIteratorFnId),
    Link(KernLinkFnId),
    MMap(KernMMapFnId),
    Mutex(KernMutexFnId),
    TimeInst(KernTimeInstFnId),
    Path(KernPathFnId),
    OsEntity(KernOsEntFnId),
    OsUser(KernOsUserFnId),
    OsGroup(KernOsGroupFnId),
    Proc(KernProcFnId),
    Thread(KernThreadFnId)
}

impl KernFnPath {
    /**
     * Returns the current function class variant as `u16`
     */
    pub fn raw_fn_class(&self) -> u16 {
        match self {
            Self::ObjConfig(_) => 0,
            Self::TaskConfig(_) => 1,
            Self::OsEntConfig(_) => 2,
            Self::Object(_) => 3,
            Self::Task(_) => 4,
            Self::Device(_) => 5,
            Self::Dir(_) => 6,
            Self::File(_) => 7,
            Self::IpcChan(_) => 8,
            Self::Iterator(_) => 9,
            Self::Link(_) => 10,
            Self::MMap(_) => 11,
            Self::Mutex(_) => 12,
            Self::TimeInst(_) => 13,
            Self::Path(_) => 14,
            Self::OsEntity(_) => 15,
            Self::OsUser(_) => 16,
            Self::OsGroup(_) => 17,
            Self::Proc(_) => 18,
            Self::Thread(_) => 19
        }
    }

    /**
     * Returns the current function id as `u16`
     */
    pub fn raw_fn_id(&self) -> u16 {
        match *self {
            Self::ObjConfig(fn_id) => fn_id.into(),
            Self::TaskConfig(fn_id) => fn_id.into(),
            Self::OsEntConfig(fn_id) => fn_id.into(),
            Self::Object(fn_id) => fn_id.into(),
            Self::Task(fn_id) => fn_id.into(),
            Self::Device(fn_id) => fn_id.into(),
            Self::Dir(fn_id) => fn_id.into(),
            Self::File(fn_id) => fn_id.into(),
            Self::IpcChan(fn_id) => fn_id.into(),
            Self::Iterator(fn_id) => fn_id.into(),
            Self::Link(fn_id) => fn_id.into(),
            Self::MMap(fn_id) => fn_id.into(),
            Self::Mutex(fn_id) => fn_id.into(),
            Self::TimeInst(fn_id) => fn_id.into(),
            Self::Path(fn_id) => fn_id.into(),
            Self::OsEntity(fn_id) => fn_id.into(),
            Self::OsUser(fn_id) => fn_id.into(),
            Self::OsGroup(fn_id) => fn_id.into(),
            Self::Proc(fn_id) => fn_id.into(),
            Self::Thread(fn_id) => fn_id.into()
        }
    }
}

impl TryFrom<(usize, usize)> for KernFnPath {
    type Error = ();

    fn try_from((variant, value): (usize, usize)) -> Result<Self, Self::Error> {
        match variant {
            0 => {
                if let Ok(fn_id) = KernObjConfigFnId::try_from(value as u16) {
                    Ok(Self::ObjConfig(fn_id))
                } else {
                    Err(())
                }
            },
            1 => {
                if let Ok(fn_id) = KernTaskConfigFnId::try_from(value as u16) {
                    Ok(Self::TaskConfig(fn_id))
                } else {
                    Err(())
                }
            },
            2 => {
                if let Ok(fn_id) = KernOsEntConfigFnId::try_from(value as u16) {
                    Ok(Self::OsEntConfig(fn_id))
                } else {
                    Err(())
                }
            },
            3 => {
                if let Ok(fn_id) = KernObjectFnId::try_from(value as u16) {
                    Ok(Self::Object(fn_id))
                } else {
                    Err(())
                }
            },
            4 => {
                if let Ok(fn_id) = KernTaskFnId::try_from(value as u16) {
                    Ok(Self::Task(fn_id))
                } else {
                    Err(())
                }
            },
            5 => {
                if let Ok(fn_id) = KernDeviceFnId::try_from(value as u16) {
                    Ok(Self::Device(fn_id))
                } else {
                    Err(())
                }
            },
            6 => {
                if let Ok(fn_id) = KernDirFnId::try_from(value as u16) {
                    Ok(Self::Dir(fn_id))
                } else {
                    Err(())
                }
            },
            7 => {
                if let Ok(fn_id) = KernFileFnId::try_from(value as u16) {
                    Ok(Self::File(fn_id))
                } else {
                    Err(())
                }
            },
            8 => {
                if let Ok(fn_id) = KernIpcChanFnId::try_from(value as u16) {
                    Ok(Self::IpcChan(fn_id))
                } else {
                    Err(())
                }
            },
            9 => {
                if let Ok(fn_id) = KrnIteratorFnId::try_from(value as u16) {
                    Ok(Self::Iterator(fn_id))
                } else {
                    Err(())
                }
            },
            10 => {
                if let Ok(fn_id) = KernLinkFnId::try_from(value as u16) {
                    Ok(Self::Link(fn_id))
                } else {
                    Err(())
                }
            },
            11 => {
                if let Ok(fn_id) = KernMMapFnId::try_from(value as u16) {
                    Ok(Self::MMap(fn_id))
                } else {
                    Err(())
                }
            },
            12 => {
                if let Ok(fn_id) = KernMutexFnId::try_from(value as u16) {
                    Ok(Self::Mutex(fn_id))
                } else {
                    Err(())
                }
            },
            13 => {
                if let Ok(fn_id) = KernTimeInstFnId::try_from(value as u16) {
                    Ok(Self::TimeInst(fn_id))
                } else {
                    Err(())
                }
            },
            14 => {
                if let Ok(fn_id) = KernPathFnId::try_from(value as u16) {
                    Ok(Self::Path(fn_id))
                } else {
                    Err(())
                }
            },
            15 => {
                if let Ok(fn_id) = KernOsEntFnId::try_from(value as u16) {
                    Ok(Self::OsEntity(fn_id))
                } else {
                    Err(())
                }
            },
            16 => {
                if let Ok(fn_id) = KernOsUserFnId::try_from(value as u16) {
                    Ok(Self::OsUser(fn_id))
                } else {
                    Err(())
                }
            },
            17 => {
                if let Ok(fn_id) = KernOsGroupFnId::try_from(value as u16) {
                    Ok(Self::OsGroup(fn_id))
                } else {
                    Err(())
                }
            },
            18 => {
                if let Ok(fn_id) = KernProcFnId::try_from(value as u16) {
                    Ok(Self::Proc(fn_id))
                } else {
                    Err(())
                }
            },
            19 => {
                if let Ok(fn_id) = KernThreadFnId::try_from(value as u16) {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObjConfig(code) => write!(f, "KernFnPath::ObjConfig({:?})", code),
            Self::TaskConfig(code) => write!(f, "KernFnPath::TaskConfig({:?})", code),
            Self::OsEntConfig(code) => write!(f, "KernFnPath::OSEntConfig({:?})", code),
            Self::Object(code) => write!(f, "KernFnPath::Object({:?})", code),
            Self::Task(code) => write!(f, "KernFnPath::Task({:?})", code),
            Self::Device(code) => write!(f, "KernFnPath::Device({:?})", code),
            Self::Dir(code) => write!(f, "KernFnPath::Dir({:?})", code),
            Self::File(code) => write!(f, "KernFnPath::File({:?})", code),
            Self::IpcChan(code) => write!(f, "KernFnPath::IpcChan({:?})", code),
            Self::Iterator(code) => write!(f, "KernFnPath::Iterator({:?})", code),
            Self::Link(code) => write!(f, "KernFnPath::Link({:?})", code),
            Self::MMap(code) => write!(f, "KernFnPath::MMap({:?})", code),
            Self::Mutex(code) => write!(f, "KernFnPath::Mutex({:?})", code),
            Self::TimeInst(code) => write!(f, "KernFnPath::Time({:?})", code),
            Self::Path(code) => write!(f, "KernFnPath::Path({:?})", code),
            Self::OsEntity(code) => write!(f, "KernFnPath::OSEntity({:?})", code),
            Self::OsUser(code) => write!(f, "KernFnPath::OSUser({:?})", code),
            Self::OsGroup(code) => write!(f, "KernFnPath::OSGroup({:?})", code),
            Self::Proc(code) => write!(f, "KernFnPath::Proc({:?})", code),
            Self::Thread(code) => write!(f, "KernFnPath::Thread({:?})", code)
        }
    }
}