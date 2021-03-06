/*! `Thread` specific entry point data */

use crate::{
    object::info::ObjUseInstant,
    task::{
        exit_status::TaskExitStatus,
        TaskId
    }
};

/**
 * Internal C entry point prototype for `Thread`s
 */
pub type CThreadEntry = extern "C" fn() -> !;

/**
 * Rust `Thread`'s user entry point prototype
 */
pub type RUserThreadEntry = fn(UserThreadArg, TaskId) -> TaskExitStatus;

/**
 * Rust entry point for user threads expects this type of argument
 */
pub type UserThreadArg = *const ();

/**
 * Rust `Thread`'s entry point for `Object::watch()` callbacks
 */
pub type RWatchThreadEntry = fn(ObjUseInstant, TaskId) -> bool;

/**
 * Rust `Thread`'s entry point for `Thread::add_cleaner()` callbacks
 */
pub type RCleanerThreadEntry = fn(TaskId);

/**
 * Context dependent `Thread`'s execution data.
 *
 * Each variant contains the executable entry-point and the data needed by
 * the context that represents
 */
#[derive(Debug)]
pub enum ThreadEntryData {
    /**
     * Data to execute a user `Thread` using `Thread::spawn()`
     */
    User {
        m_entry_point: RUserThreadEntry,
        m_entry_arg: UserThreadArg,
        m_thread_id: TaskId
    },

    /**
     * Data to execute a `Object::watch()` callback
     */
    WatchCallback {
        m_entry_point: RWatchThreadEntry,
        m_entry_arg: ObjUseInstant,
        m_thread_id: TaskId
    },

    /**
     * Data to execute a `Thread::add_cleaner()` callback
     */
    CleanerCallback {
        m_entry_point: RCleanerThreadEntry,
        m_thread_id: TaskId
    },

    /**
     * Default value, usable only for un-initialized `ThreadEntryData`
     */
    None
}

impl Default for ThreadEntryData {
    fn default() -> Self {
        Self::None
    }
}
