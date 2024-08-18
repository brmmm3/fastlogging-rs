use std::{
    process::{self, Command},
    thread,
    time::Duration,
};

use fastlogging::{get_parent_pid, LoggingError};

fn run_parent(child: u32) -> Result<(), LoggingError> {
    println!("# {} Run parent. Child has pid {child}.", process::id());
    fastlogging::debug("Debug Message from parent")?;
    fastlogging::info("Info Message from parent")?;
    fastlogging::warning("Warning Message from parent")?;
    fastlogging::error("Error Message from parent")?;
    fastlogging::fatal("Fatal Message from parent")?;
    println!("# {} Parent finished", process::id());
    Ok(())
}

fn run_child(ppid: u32) -> Result<(), LoggingError> {
    println!("# {} Run child. Parent has pid {ppid}.", process::id());
    thread::sleep(Duration::from_millis(20));
    fastlogging::debug("Debug Message from child")?;
    fastlogging::info("Info Message from child")?;
    fastlogging::warning("Warning Message from child")?;
    fastlogging::error("Error Message from child")?;
    fastlogging::fatal("Fatal Message from child")?;
    println!("# {} Child finished", process::id());
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    println!("\n# {} Start main", process::id());
    //fastlogging::set_debug(3);
    if let Some(ppid) = get_parent_pid() {
        // This is the child
        run_child(ppid)?;
    } else {
        // This is the parent. Create child process.
        let mut child = Command::new(std::env::current_exe()?)
            .spawn()
            .expect("failed to execute child");
        run_parent(child.id())?;
        child.wait()?;
    }
    println!("# {} Continue main", process::id());
    fastlogging::debug("Debug Message from main")?;
    thread::sleep(Duration::from_millis(10));
    println!("# {} SyncAll main", process::id());
    fastlogging::sync_all(0.1)?;
    println!("# {} Shutdown main", process::id());
    fastlogging::shutdown(false)?;
    println!("# {} Finished main\n", process::id());
    Ok(())
}
