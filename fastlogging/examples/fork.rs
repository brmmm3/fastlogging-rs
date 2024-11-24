use std::{process, thread, time::Duration};

use fastlogging::{root, LoggingError};
use fork::{fork, Fork};

fn run_parent(child: i32) -> Result<(), LoggingError> {
    println!("# Run parent. Child has pid {child}.");
    root::debug("Debug Message from parent")?;
    root::info("Info Message from parent")?;
    root::warning("Warning Message from parent")?;
    root::error("Error Message from parent")?;
    root::fatal("Fatal Message from parent")?;
    println!("# Parent finished");
    Ok(())
}

fn run_child() -> Result<(), LoggingError> {
    println!("# Run child with pid {}", process::id());
    thread::sleep(Duration::from_millis(20));
    root::debug("Debug Message from child")?;
    root::info("Info Message from child")?;
    root::warning("Warning Message from child")?;
    root::error("Error Message from child")?;
    root::fatal("Fatal Message from child")?;
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
    root::debug("Debug Message from main")?;
    println!("# main finished");
    thread::sleep(Duration::from_millis(100));
    Ok(())
}
