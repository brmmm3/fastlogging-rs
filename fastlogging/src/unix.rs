use std::os::unix::process;

pub fn getppid() -> u32 {
    process::parent_id()
}
