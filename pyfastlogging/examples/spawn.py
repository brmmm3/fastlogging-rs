import os
import time

import pyfastlogging as fl


def run_parent():
    print("# Run parent.")
    fl.debug("Debug Message from parent")
    fl.info("Info Message from parent")
    fl.warning("Warning Message from parent")
    fl.error("Error Message from parent")
    fl.fatal("Fatal Message from parent")
    print("# Parent finished")


def run_child(ppid: int):
    print(f"# Run child with pid {os.getpid()}. Parent has pid {ppid}.")
    time.sleep(0.02)
    fl.debug("Debug Message from child")
    fl.info("Info Message from child")
    fl.warning("Warning Message from child")
    fl.error("Error Message from child")
    fl.fatal("Fatal Message from child")
    print("# Child finished")


if __name__ == "__main__":
    import multiprocessing

    multiprocessing.set_start_method("spawn")
    print(f"# Start main with pid {os.getpid()}")
    if ppid := os.fork():
        run_child(ppid)
    else:
        run_parent()
    print(f"# Continue main with pid {os.getpid()}")
    fl.debug("Debug Message from main")
    print("# main finished")
    time.sleep(0.1)
