//! Shared plumbing for every VIctory crate and tool.
//!
//! * [`config`]: where the user's files live (never inside the repository).
//! * [`policy`]: the runtime half of rule #1, refusing to write
//!   game-derived data into the source tree.
//! * [`logging`]: one tracing setup for all binaries.

pub mod config;
pub mod logging;
pub mod policy;

pub use config::Config;
pub use policy::{PolicyError, ensure_outside_repo};

std::thread_local! {
    static TRY_COPY_DEPTH: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// True while the current thread is inside an intentional SEH-guarded copy
/// (`victory_try_copy`) so first-chance exception loggers can stay silent.
pub fn in_try_copy() -> bool {
    TRY_COPY_DEPTH.with(|depth| depth.get() > 0)
}

/// Runs `f` with [`in_try_copy`] set on the current thread.
pub fn with_try_copy<R>(f: impl FnOnce() -> R) -> R {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            TRY_COPY_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
        }
    }
    TRY_COPY_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    let _reset = Reset;
    f()
}
