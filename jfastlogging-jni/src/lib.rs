use jni::errors::{Result as JniResult, ThrowRuntimeExAndDefault};

mod def;
mod macros;
pub use def::extConfigNew;
mod logging;
mod writer;
use jni::{Env, EnvUnowned};
pub use logging::*;
mod logger;
pub use logger::*;

fn enter_jni<'local, T: Default>(
    mut env: EnvUnowned<'local>,
    f: impl FnOnce(&mut Env) -> JniResult<T>,
) -> T {
    env.with_env(f).resolve::<ThrowRuntimeExAndDefault>()
}
