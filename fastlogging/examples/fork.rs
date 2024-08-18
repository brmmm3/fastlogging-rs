use std::{process, thread, time::Duration};

use fastlogging::LoggingError;
use fork::{fork, Fork};

fn run_parent(child: i32) -> Result<(), LoggingError> {
    println!("# Run parent. Child has pid {child}.");
    fastlogging::debug("Debug Message from parent")?;
    fastlogging::info("Info Message from parent")?;
    fastlogging::warning("Warning Message from parent")?;
    fastlogging::error("Error Message from parent")?;
    fastlogging::fatal("Fatal Message from parent")?;
    println!("# Parent finished");
    Ok(())
}

fn run_child() -> Result<(), LoggingError> {
    println!("# Run child with pid {}", process::id());
    thread::sleep(Duration::from_millis(20));
    fastlogging::debug("Debug Message from child")?;
    fastlogging::info("Info Message from child")?;
    fastlogging::warning("Warning Message from child")?;
    fastlogging::error("Error Message from child")?;
    fastlogging::fatal("Fatal Message from child")?;
    println!("# Child finished");
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    println!("# Start main with pid {}", process::id());
    match fork() {
        Ok(Fork::Parent(child)) => run_parent(child)?,
        Ok(Fork::Child) => run_child()?,
        Err(_) => println!("Fork failed"),
    }
    println!("# Continue main with pid {}", process::id());
    fastlogging::debug("Debug Message from main")?;
    println!("# main finished");
    thread::sleep(Duration::from_millis(100));
    Ok(())
}
