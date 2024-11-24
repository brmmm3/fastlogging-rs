use std::{
    process::{self, Command},
    thread,
    time::Duration,
};

use fastlogging::{root, LoggingError};

fn run_parent(child: u32) -> Result<(), LoggingError> {
    println!("# {} Run parent. Child has pid {child}.", process::id());
    root::debug("Debug Message from parent")?;
    root::info("Info Message from parent")?;
    root::warning("Warning Message from parent")?;
    root::error("Error Message from parent")?;
    root::fatal("Fatal Message from parent")?;
    println!("# {} Parent finished", process::id());
    Ok(())
}

fn run_child(ppid: u32) -> Result<(), LoggingError> {
    println!("# {} Run child. Parent has pid {ppid}.", process::id());
    thread::sleep(Duration::from_millis(20));
    root::debug("Debug Message from child")?;
    root::info("Info Message from child")?;
    root::warning("Warning Message from child")?;
    root::error("Error Message from child")?;
    root::fatal("Fatal Message from child")?;
    println!("# {} Child finished", process::id());
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    println!("\n# {} Start main", process::id());
    //fastlogging::set_debug(3);
    if let Some(ppid) = root::get_parent_pid() {
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
    root::debug("Debug Message from main")?;
    thread::sleep(Duration::from_millis(10));
    println!("# {} SyncAll main", process::id());
    root::sync_all(0.1)?;
    println!("# {} Shutdown main", process::id());
    root::shutdown(false)?;
    println!("# {} Finished main\n", process::id());
    Ok(())
}
