/*! Context wide limit constants */

/**
 * Maximum amount of arguments that the kernel accepts
 */
pub const SYSCALL_ARGS_COUNT_MAX: usize = 6;

/**
 * Maximum length in for a filesystem path managed by the VFS Kernel
 * module
 */
pub const VFS_PATH_LEN_MAX: usize = 1024;

/**
 * Maximum length in bytes for a single name in a filesystem path
 */
pub const VFS_NAME_LEN_MAX: usize = 256;

/**
 * Maximum amount of threads that could call `Object::watch()` at the same
 * time for the same object
 */
pub const OBJ_WATCHERS_COUNT_MAX: usize = 64;

/**
 * Maximum amount of open `LibApi::objs::Object` instance for each process
 */
pub const OBJ_OPENED_COUNT_MAX: usize = 1024;

/**
 * Maximum length in bytes for an `OsEntity` name
 */
pub const OS_ENTITY_NAME_LEN_MAX: usize = 64;

/**
 * Maximum amount of `OsGroup`s for each `OsUser`
 */
pub const OS_USER_GROUPS_COUNT_MAX: usize = 64;

/**
 * Maximum length in bytes for a `Thread` name
 */
pub const THREAD_NAME_LEN_MAX: usize = 32;

/**
 * Maximum amount of single arguments for a process
 */
pub const PROC_ARG_COUNT_MAX: usize = 32;

/**
 * Maximum length in bytes for each process argument
 */
pub const PROC_ARG_LEN_MAX: usize = 64;

/**
 * Maximum length in bytes for the error message in `OsError`
 */
pub const OS_ERROR_MESSAGE_LEN_MAX: usize = 64;
