import os
import time

import fastlogging_rs as fl

# fl.set_debug(3)


def run_parent():
    print("# Run parent.")
    fl.set_domain(f"parent")
    fl.debug("Debug Message from parent")
    fl.info("Info Message from parent")
    fl.warning("Warning Message from parent")
    fl.error("Error Message from parent")
    fl.fatal("Fatal Message from parent")
    print("# Parent finished")


def run_child(ppid: int):
    pid = os.getpid()
    print(f"# Run child with pid {pid}. Parent has pid {ppid}.")
    fl.set_domain(f"child{pid}")
    fl.debug(f"Debug Message from child {pid}")
    fl.info(f"Info Message from child {pid}")
    fl.warning(f"Warning Message from child {pid}")
    fl.error(f"Error Message from child {pid}")
    fl.fatal(f"Fatal Message from child {pid}")
    fl.sync()
    time.sleep(0.5)
    print(f"# Child finished {pid}")


if __name__ == "__main__":
    import multiprocessing
    from multiprocessing import Pool, freeze_support

    multiprocessing.set_start_method("spawn")
    freeze_support()
    ppid = os.getpid()
    print(f"# Start main with pid {ppid}")
    cnt = 3  # os.cpu_count()
    with Pool(cnt) as pool:
        results = []
        for _ in range(cnt):
            results.append(pool.apply_async(run_child, (ppid,)))
        run_parent()
        for result in results:
            result.get()
    fl.debug("Debug Message from main")
    fl.sync()
    time.sleep(0.1)
    print("# main finished")
