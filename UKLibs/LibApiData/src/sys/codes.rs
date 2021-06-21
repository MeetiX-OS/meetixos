/*! Kernel function call codes */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the system call codes for the `KernHandle` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernHandleFnId {
    IsValid,
    Drop
}

/**
 * Lists the system call codes for the `ObjConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernObjConfigFnId {
    ApplyConfig
}

/**
 * Lists the system call codes for the `TaskConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTaskConfigFnId {
    CreateTask,
    InitFind
}

/**
 * Lists the system call codes for the `OSEntConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsEntConfigFnId {
    CreateEntity,
    InitFind
}

/**
 * Lists the system call codes for the `Object` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernObjectFnId {
    AddRef,
    DropName,
    Info,
    UpdateInfo,
    Send,
    Recv,
    Watch,
    IsValid
}

/**
 * Lists the system call codes for the `Task` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTaskFnId {
    This,
    Terminate,
    Yield,
    IsAlive
}

/**
 * Lists the system call codes for the `Device` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernDeviceFnId {
    Read,
    Write,
    SetPos,
    MapToMem,
    IOSetup
}

/**
 * Lists the system call codes for the `Dir` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernDirFnId {
    InitIter
}

/**
 * Lists the system call codes for the `File` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernFileFnId {
    ReadData,
    WriteData,
    Copy,
    Move,
    SetPos,
    MapToMem
}

/**
 * Lists the system call codes for the `IpcChan` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernIpcChanFnId {
    Send,
    Recv
}

/**
 * Lists the system call codes for the `KrnIterator`
 * struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KrnIteratorFnId {
    NextValue,
    SetBeginToEndPos,
    SetEndToBeginPos
}

/**
 * Lists the system call codes for the `Link` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernLinkFnId {
    Deref,
    ReferTo
}

/**
 * Lists the system call codes for the `MMap` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernMMapFnId {
    GetPtr,
    DropPtr,
    IsFile,
    IsDevice
}

/**
 * Lists the system call codes for the `Mutex` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernMutexFnId {
    Lock,
    TryLock,
    Unlock,
    IsLocked
}

/**
 * Lists the system call codes for the `Instant` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTimeInstFnId {
    Now
}

/**
 * Lists the system call codes for the `Path` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernPathFnId {
    Exists
}

/**
 * Lists the system call codes for the `OsEntity` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsEntFnId {
    Name
}

/**
 * Lists the system call codes for the `OsUser`
 * struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsUserFnId {
    Groups
}

/**
 * Lists the system call codes for the `OsGroup`
 * struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsGroupFnId {
    AddUser
}

/**
 * Lists the system call codes for the `Proc` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernProcFnId {
    MainThread,
    Mount,
    UnMount
}

/**
 * Lists the system call codes for the `Thread` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernThreadFnId {
    WaitFor,
    AddCleaner,
    CallbackReturn,
    GetEntryData
}
