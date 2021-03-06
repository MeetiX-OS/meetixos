/*! Kernel call error management */

use core::fmt;

use helps::str::{
    copy_str_to_u8_buf,
    u8_slice_to_str_slice
};

use crate::{
    error::class::OsErrorClass,
    limit::OS_ERROR_MESSAGE_LEN_MAX,
    sys::{
        fn_path::KernFnPath,
        RawKernHandle,
        TAsSysCallPtr
    },
    task::TaskId
};

pub mod class;

/**
 * Operating system error in MeetiX
 */
#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct OsError {
    m_class: OsErrorClass,
    m_kern_fn_path: KernFnPath,
    m_inst_handle: Option<RawKernHandle>,
    m_proc_id: TaskId,
    m_thread_id: TaskId,
    m_message: Option<[u8; OS_ERROR_MESSAGE_LEN_MAX]>
}

impl OsError /* Constructors */ {
    /**
     * Constructs an `OsError` filled with the given data
     */
    pub fn new(class: OsErrorClass,
               kern_fn_path: KernFnPath,
               inst_handle: Option<RawKernHandle>,
               proc_id: TaskId,
               thread_id: TaskId,
               message: Option<&str>)
               -> Self {
        Self { m_class: class,
               m_kern_fn_path: kern_fn_path,
               m_inst_handle: inst_handle,
               m_proc_id: proc_id,
               m_thread_id: thread_id,
               m_message: message.map(|str_buf| {
                                     let mut buffer = [0; OS_ERROR_MESSAGE_LEN_MAX];
                                     copy_str_to_u8_buf(&mut buffer, str_buf);
                                     buffer
                                 }) }
    }
}

impl OsError /* Getters */ {
    /**
     * Returns the `ErrorClass`
     */
    pub fn error_class(&self) -> OsErrorClass {
        self.m_class
    }

    /**
     * Returns the `KernFnPath` which originates this `OsError`
     */
    pub fn kern_fn_path(&self) -> KernFnPath {
        self.m_kern_fn_path
    }

    /**
     * Returns the `KernHandle` which originates this `OsError` if any
     */
    pub fn inst_handle(&self) -> Option<RawKernHandle> {
        self.m_inst_handle
    }

    /**
     * Returns the formatted message of the error if any
     */
    pub fn message(&self) -> Option<&str> {
        self.m_message.as_ref().map(|message_buf| u8_slice_to_str_slice(message_buf))
    }
}

impl TAsSysCallPtr for OsError {
    /* No methods to implement */
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /* write the complete error message as follow:
         * [<pid>:<tid>] Error: <Human readable error class>\n
         *       : Reason: <Optional error message from the Kernel>
         */
        writeln!(f,
                 "[{}:{}] Error: {} - {}",
                 self.m_proc_id, self.m_thread_id, self.m_kern_fn_path, self.m_class)?;
        if let Some(message) = self.message() {
            writeln!(f, "\t: Reason: {}", message)?;
        }
        Ok(())
    }
}
