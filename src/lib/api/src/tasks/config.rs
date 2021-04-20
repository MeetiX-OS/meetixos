/*! # `Task` Configuration
 *
 * Implements the standard and unique way to find existing [`Task`]s or
 * spawn new one
 *
 * [`Task`]: /api/tasks/trait.Task.html
 */

use core::marker::PhantomData;

use os::sysc::{codes::KernTaskConfigFnId, fn_path::KernFnPath};

use crate::{
    bits::task::{SchedPolicy, TaskCpu, TaskPrio, TaskSpecData},
    caller::{KernCaller, Result},
    config::{ConfigFinderIter, ConfigMode, CreatMode, FindMode},
    ents::impls::{OSGroup, OSUser},
    objs::impls::File,
    tasks::{
        impls::{Proc, Thread},
        Task, TaskId
    }
};

/** # `Task` Configuration
 *
 * Implements a functional standard interface to find existing [`Task`] or
 * spawn new one.
 *
 * This object and his interface take place of the old style Unix's
 * [`fork()`]/[`clone()`]/[`exec()`]/[`pthread_*`]/[`sched_*`] system calls,
 * providing a function-call-chain interface where each method enables or
 * customizes an execution feature of the new [`Task`] in [`CreatMode`] or
 * enables/specifies a filter in [`FindMode`].
 *
 * The configuration in [`CreatMode`] can be run with:
 * * [`TaskConfig::<Thread, CreatMode>::run()`] - to spawn a new thread for
 *   the current caller process
 * * [`TaskConfig::<Proc, CreatMode>::run()`] - to spawn a new process that
 *   executes a new executable program
 *
 * Otherwise, in [`FindMode`], can be launched with
 * [`TaskConfig::<FindMode>::search()`]
 *
 * [`Task`]: /api/tasks/trait.Task.html
 * [`fork()`]: https://man7.org/linux/man-pages/man2/fork.2.html
 * [`clone()`]: https://man7.org/linux/man-pages/man2/clone.2.html
 * [`exec()`]: https://man7.org/linux/man-pages/man2/execve.2.html
 * [`pthread_*`]: https://linux.die.net/man/7/pthreads
 * [`sched_*`]: https://www.man7.org/linux/man-pages/man7/sched.7.html
 * [`TaskConfig::with_prio()`]:
 * /api/tasks/struct.TaskConfig.html#method.with_prio
 * [`TaskConfig::<Thread, CreatMode>::run()`]:
 * /api/tasks/struct.TaskConfig.html#method.run-1
 * [`TaskConfig::<Proc, CreatMode>::run()`]:
 * /api/tasks/struct.TaskConfig.html#method.run
 * [`TaskConfig::<FindMode>::search()`]:
 * /api/tasks/struct.TaskConfig.html#method.search
 * [`CreatMode`]: /api/config/struct.CreatMode.html
 * [`FindMode`]: /api/config/struct.FindMode.html
 */
#[derive(Debug)]
pub struct TaskConfig<T, M>
    where T: Task,
          M: ConfigMode {
    m_id: Option<u32>,
    m_children_of: Option<T>,
    m_sched_policy: SchedPolicy,
    m_prio: TaskPrio,
    m_cpu: TaskCpu,
    m_spec: TaskSpecData,
    m_os_user: Option<OSUser>,
    m_os_group: Option<OSGroup>,
    _unused: PhantomData<T>,
    _unused2: PhantomData<M>
}

impl<T, M> TaskConfig<T, M>
    where T: Task,
          M: ConfigMode
{
    /** # Constructs a new `TaskConfig`
     *
     * The instance is blank and zeroed
     */
    pub(crate) fn new() -> Self {
        Self { m_id: None,
               m_children_of: None,
               m_sched_policy: SchedPolicy::Preemptive,
               m_prio: TaskPrio::Normal,
               m_cpu: TaskCpu::Any,
               m_spec: TaskSpecData::None,
               m_os_user: None,
               m_os_group: None,
               _unused: Default::default(),
               _unused2: Default::default() }
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl<T: Task, M: ConfigMode> TaskConfig<T, M> {
    /** Returns the optional id for searching
     */
    pub fn id(&self) -> Option<u32> {
        self.m_id
    }

    /** Returns the [`Task`] to use as parent for searching
     *
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn children_parent(&self) -> Option<&T> {
        self.m_children_of.as_ref()
    }

    /** Returns the [`SchedPolicy`] chosen for the new [`Task`]
     *
     * [`SchedPolicy`]: /api/bits/task/enum.SchedPolicy.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn sched_policy(&self) -> SchedPolicy {
        self.m_sched_policy
    }

    /** Returns the [`TaskCpu`] chosen for the new [`Task`]
     *
     * [`TaskCpu`]: /api/bits/task/enum.TaskCpu.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn task_cpu(&self) -> TaskCpu {
        self.m_cpu
    }

    /** Returns the reference to the [`TaskSpecData`]
     *
     * [`TaskSpecData`]: /api/bits/task/enum.TaskSpecData.html
     */
    pub fn task_spec_data(&self) -> &TaskSpecData {
        &self.m_spec
    }

    /** Returns the optional [`OSUser`] chosen
     *
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     */
    pub fn os_user(&self) -> Option<OSUser> {
        self.m_os_user
    }

    /** Returns the optional [`OSGroup`] chosen
     *
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     */
    pub fn os_group(&self) -> Option<OSGroup> {
        self.m_os_group
    }
}

impl<T> TaskConfig<T, CreatMode> where T: Task {
    /** # Specifies the scheduling policy
     *
     * The variant of [`SchedPolicy`] given tells to the kernel which
     * scheduling algorithm must be used for the new [`Task`]
     *
     * [`SchedPolicy`]: /api/bits/task/enum.SchedPolicy.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn with_sched_policy(&mut self, sched_policy: SchedPolicy) -> &mut Self {
        self.m_sched_policy = sched_policy;
        self
    }

    /** # Specifies the priority class
     *
     * The variant of [`TaskPrio`] given tells to the kernel which is the
     * class of priority that the new [`Task`] must have for all his
     * execution life
     *
     * [`TaskPrio`]: /api/bits/task/enum.TaskPrio.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn with_prio(&mut self, priority: TaskPrio) -> &mut Self {
        self.m_prio = priority;
        self
    }

    /** # Specifies the CPU affinity
     *
     * The variant of [`TaskCpu`] given tells to the kernel whether and how
     * the new [`Task`] must be affine to a subset of the available CPUs
     * in a SMP environment
     *
     * [`TaskCpu`]: /api/bits/task/enum.TaskCpu.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn with_cpu(&mut self, cpu: TaskCpu) -> &mut Self {
        self.m_cpu = cpu;
        self
    }

    /** # Runs a new `Task`
     *
     * Requests to the kernel to apply the given configuration to spawn a
     * new [`Task`]
     *
     * [`Task`]: /api/tasks/trait.Task.html
     */
    fn run_task(self) -> Result<T> {
        self.kern_call_1(KernFnPath::TaskConfig(KernTaskConfigFnId::CreateTask),
                         &self as *const _ as usize)
            .map(|task_id| T::from(TaskId::from(task_id)))
    }
}

impl<T> TaskConfig<T, FindMode> where T: Task {
    /** # Specifies the `Task`'s ID
     *
     * Tells to the kernel exactly which task is requested.
     *
     * Any other filter is ignored when this one is enabled
     */
    pub fn with_id(&mut self, id: u32) -> &mut Self {
        self.m_id = Some(id);
        self
    }

    /** # Specifies the parent `Task`
     *
     * Enables the parent [`Task`] filter to tell the kernel on which
     * children iterate
     *
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn children_of(&mut self, task: T) -> &mut Self {
        self.m_children_of = Some(task);
        self
    }

    /** # Specifies the owner `OSUser`
     *
     * Enables the owner [`OSUser`] filter to tell the kernel which
     * [`Task`]s select
     *
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn owned_by_user(&mut self, user: OSUser) -> &mut Self {
        self.m_os_user = Some(user);
        self
    }

    /** # Specifies the owner `OSGroup`
     *
     * Enables the owner [`OSGroup`] filter to tell the kernel which
     * [`Task`]s select
     *
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     * [`Task`]: /api/tasks/trait.Task.html
     */
    pub fn owned_by_group(&mut self, group: OSGroup) -> &mut Self {
        self.m_os_group = Some(group);
        self
    }

    /** # Searches for existing `Task`s
     *
     * Dispatches the configuration to the kernel to validate and initialize
     * an iteration pool on which the returned [`Iterator`] will fetch
     * the results.
     *
     * If the given configuration have no filters, the kernel initializes an
     * iteration pool with **ALL** the active tasks of the `T` type
     * ([`Proc`] or [`Thread`])
     *
     * [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    pub fn search(self) -> Result<impl Iterator<Item = T>> {
        self.kern_call_1(KernFnPath::TaskConfig(KernTaskConfigFnId::InitFind),
                         &self as *const _ as usize)
            .map(|iter_id| ConfigFinderIter::from(iter_id))
    }
}

impl TaskConfig<Proc, CreatMode> {
    /** # Specifies the owner `OSUser`
     *
     * Overrides the owner [`OSUser`] which, otherwise, will be inherited by
     * the parent [`Proc`]
     *
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     */
    pub fn owned_by_user(&mut self, user: OSUser) -> &mut Self {
        self.m_os_user = Some(user);
        self
    }

    /** # Specifies the owner `OSGroup`
     *
     * Overrides the owner [`OSGroup`] which, otherwise, will be inherited
     * by the parent [`Proc`]
     *
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     */
    pub fn owned_by_group(&mut self, group: OSGroup) -> &mut Self {
        self.m_os_group = Some(group);
        self
    }

    /** # Spawns a new `Proc`
     *
     * Dispatches this spawner configuration to the kernel that creates a
     * new [`Proc`].
     *
     * The new [`Proc`] executes the given [`File`] with the given optional
     * arguments.
     *
     * The [`File`] must be a valid executable file format, and must be
     * [opened] with [read]/[execute] options enabled
     *
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     * [`File`]: /api/objs/impls/struct.File.html
     * [opened]: /api/objs/struct.ObjConfig.html
     * [read]: /api/objs/struct.ObjConfig.html#method.for_read
     * [execute]: /api/objs/struct.ObjConfig.html#method.for_exec
     */
    pub fn run(mut self, file: File, args: Option<&[&str]>) -> Result<Proc> {
        self.m_spec = TaskSpecData::new_proc(file, args);
        self.run_task()
    }
}

impl TaskConfig<Proc, FindMode> {
    /** # Specifies the executed `File`
     *
     * Enables the executed [`File`] filter to tell the kernel on which
     * [`Proc`] iterate
     *
     * [`File`]: /api/objs/impls/struct.File.html
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     */
    pub fn executor_of(&mut self, file: File) -> &mut Self {
        self.m_spec = TaskSpecData::new_proc(file, None);
        self
    }
}

impl TaskConfig<Thread, CreatMode> {
    /** # Spawns a new `Thread`
     *
     * Dispatches this spawner configuration to the kernel that creates a
     * new [`Thread`].
     *
     * The new [`Thread`] starts his execution from the given `entry_point`
     * function, and receives the given `arg` as `entry_point`'s argument.
     *
     * The function's returns value is used as [`Task::terminate()`]'s
     * argument
     *
     * The newly created [`Thread`] shares the same address space of the
     * [`Proc`] that spawns it
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     * [`Task::terminate()`]: /api/tasks/trait.Task.html#method.terminate
     */
    pub fn run(mut self,
               entry_point: fn(usize) -> bool,
               arg: usize,
               name: Option<&str>)
               -> Result<Thread> {
        self.m_spec = TaskSpecData::new_thread(Some(entry_point), Some(arg), name);
        self.run_task()
    }
}

impl TaskConfig<Thread, FindMode> {
    /** # Specifies the `Thread` name
     *
     * Enables the [`Thread`] name filter to tell the kernel on which name
     * iterate
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    pub fn with_name(&mut self, name: &str) -> &mut Self {
        self.m_spec = TaskSpecData::new_thread(None, None, Some(name));
        self
    }
}

impl<T, M> KernCaller for TaskConfig<T, M>
    where T: Task,
          M: ConfigMode
{
    /* Nothing to implement */
}
