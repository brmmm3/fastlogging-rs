import os

import fastlogging_rs as fl

fl.set_debug(3)


def run_parent():
    print("# Run parent.")
    fl.debug("Debug Message from parent")
    fl.info("Info Message from parent")
    fl.warning("Warning Message from parent")
    fl.error("Error Message from parent")
    fl.fatal("Fatal Message from parent")
    print("# Parent finished")


def run_child(ppid: int):
    pid = os.getpid()
    print(f"# Run child with pid {pid}. Parent has pid {ppid}.")
    fl.debug(f"Debug Message from child {pid}")
    fl.info(f"Info Message from child {pid}")
    fl.warning(f"Warning Message from child {pid}")
    fl.error(f"Error Message from child {pid}")
    fl.fatal(f"Fatal Message from child {pid}")
    print(f"# Child finished {pid}")


if __name__ == "__main__":
    from multiprocessing import Pool, freeze_support

    freeze_support()
    ppid = os.getpid()
    print(f"# Start main with pid {ppid}")
    cnt = 1  # os.cpu_count()
    pool = Pool(cnt)
    for _ in range(cnt):
        pool.apply(run_child, (ppid,))
    run_parent()
    pool.close()
    pool.join()
    fl.debug("Debug Message from main")
    fl.sync()
    print("# main finished")
