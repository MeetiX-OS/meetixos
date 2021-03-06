/*! Userland entry-point */

use api::task::{
    impls::proc::Proc,
    TTask
};
use api_data::{
    error::OsError,
    task::exit_status::TaskExitStatus
};

/**
 * Entry point for userspace applications
 */
#[lang = "start"]
fn lang_start<T>(rust_entry_point: fn() -> T,
                 _argc: isize,
                 _argv: *const *const u8)
                 -> isize
    where T: TTermination + 'static {
    Proc::exit(rust_entry_point().report());
}

/**
 * Termination trait useful to obtain the TaskExitStatus
 */
#[lang = "termination"]
pub trait TTermination {
    /**
     * Returns the `TaskExitStatus`
     */
    fn report(self) -> TaskExitStatus;
}

impl TTermination for () {
    fn report(self) -> TaskExitStatus {
        TaskExitStatus::Success
    }
}

impl TTermination for Result<(), OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(_) => TaskExitStatus::Success,
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl TTermination for Result<u8, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl TTermination for Result<u16, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl TTermination for Result<u32, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl TTermination for Result<u64, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl TTermination for Result<usize, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}
