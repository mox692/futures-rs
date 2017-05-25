//! Tasks used to drive a future computation
//!
//! It's intended over time a particular operation (such as servicing an HTTP
//! request) will involve many futures. This entire operation, however, can be
//! thought of as one unit, as the entire result is essentially just moving
//! through one large state machine.
//!
//! A "task" is the unit of abstraction for what is driving this state machine
//! and tree of futures forward. A task is used to poll futures and schedule
//! futures with, and has utilities for sharing data between tasks and handles
//! for notifying when a future is ready. Each task also has its own set of
//! task-local data generated by `task_local!`.
//!
//! Note that libraries typically should not manage tasks themselves, but rather
//! leave that to event loops and other "executors" (see the `executor` module),
//! or by using the `wait` method to create and execute a task directly on the
//! current thread.
//!
//! More information about the task model can be found [online at tokio.rs].
//!
//! [online at tokio.rs]: https://tokio.rs/docs/going-deeper-futures/futures-model/
//!
//! ## Functions
//!
//! There is an important bare function in this module: `current`. The
//! `current` function returns a handle to the currently running task, panicking
//! if one isn't present. This handle is then used to later notify the task that
//! it's ready to make progress through the `Task::notify` method.

#[doc(hidden)]
#[deprecated(since = "0.1.4", note = "import through the executor module instead")]
#[cfg(all(feature = "with-deprecated", feature = "use_std"))]
#[allow(deprecated)]
pub use task_impl::{Spawn, spawn, Unpark, Executor, Run, park};

pub use task_impl::{Task, current, init};

#[allow(deprecated)]
#[cfg(feature = "use_std")]
pub use task_impl::{LocalKey, with_unpark_event, UnparkEvent, EventSet};

#[doc(hidden)]
#[deprecated(since = "0.1.4", note = "import through the executor module instead")]
#[cfg(all(feature = "with-deprecated", feature = "use_std"))]
#[allow(deprecated)]
pub use task_impl::TaskRc;
