import os
import time
import logging.handlers

import json
import shutil
from fastlogging import LogInit
import fastlogging_rs as fl
from fastlogging_rs import (
    CRITICAL,
    ERROR,
    WARNING,
    INFO,
    DEBUG,
    Logging,
    Logger,
    Level2Sym,
)

MB = 1024 * 1024

tmpDirName = "C:\\temp\\pyfastlogging" if os.name == "nt" else "/tmp/pyfastlogging"


# noinspection PyShadowingNames
def LoggingWork(logger, cnt: int, bWithException: bool, message: str) -> float:
    t1 = time.time()
    for i in range(cnt):
        logger.fatal(f"Fatal {i} {message}")
        logger.error(f"Error {i} {message}")
        logger.warning(f"Warning {message} {i}")
        logger.info(f"Info {message} {i}")
        logger.debug(f"Debug {message} {i}")
        logger.fatal(f"Fatal {i} {message}")
        logger.error(f"Error {i} {message}")
        logger.warning(f"Warning {message} {i}")
        logger.info(f"Info {message} {i}")
        logger.debug(f"Debug {message} {i}")
        logger.fatal(f"Fatal {i} {message}")
        logger.error(f"Error {i} {message}")
        logger.warning(f"Warning {message} {i}")
        logger.info(f"Info {message} {i}")
        logger.debug(f"Debug {message} {i}")
        logger.fatal(f"Fatal {i} {message}")
        logger.error(f"Error {i} {message}")
        logger.warning(f"Warning {message} {i}")
        logger.info(f"Info {message} {i}")
        logger.debug(f"Debug {message} {i}")
        if bWithException:
            # noinspection PyBroadException
            try:
                # noinspection PyUnusedLocal
                x = 1 / 0
            except:
                logger.exception("EXCEPTION")
    return time.time() - t1


def GetTitle(
    prefix: str,
    msg: str,
    fileName: str | None,
    bRotate: bool,
    bWithException: bool,
    level: int,
) -> str:
    title = [msg, "FILE" if fileName else "NO FILE"]
    if bRotate:
        title.append("ROTATE")
    if bWithException:
        title.append("EXC")
    title.append(Level2Sym(level).name)
    return f"{prefix}_" + "_".join(title)


def GetPathName(tmpDirName: str, fileName: str | None, title: str) -> str | None:
    if not fileName:
        return None
    dirName = os.path.join(tmpDirName, title)
    if os.path.exists(dirName):
        shutil.rmtree(dirName)
    os.makedirs(dirName)
    return os.path.join(dirName, fileName)


# noinspection PyShadowingNames
def DoLogging(
    cnt: int,
    level: int,
    pathName: str | None,
    bRotate: bool,
    bWithException: bool,
    message: str,
) -> float:
    if pathName:
        if bRotate:
            logHandler = logging.handlers.RotatingFileHandler(
                pathName, mode="a", maxBytes=MB, backupCount=8
            )
        else:
            logHandler = logging.FileHandler(pathName)
    else:
        logHandler = logging.NullHandler()
    logFormatter = logging.Formatter(
        "%(asctime)-15s %(name)s %(levelname)-8.8s %(message)s", "%Y.%m.%d %H:%M:%S"
    )
    logHandler.setFormatter(logFormatter)
    logHandler.setLevel(level)
    logger = logging.getLogger("root")
    logger.addHandler(logHandler)
    logger.setLevel(level)
    t1 = time.time()
    dt0 = LoggingWork(logger, cnt, bWithException, message)
    logHandler.close()
    dt = time.time() - t1
    print(f"  total: {dt0: .3f} {dt: .3f}")
    return dt


# noinspection PyShadowingNames
def DoFastLogging(
    cnt: int,
    level: int,
    pathName: str | None,
    bRotate: bool,
    bWithException: bool,
    message: str,
    bThreads: bool,
) -> float:
    if bRotate:
        size = MB
        count = 8
    else:
        size = 0
        count = 0
    logger = LogInit(
        "main",
        level,
        pathName,
        size,
        count,
        False,
        False,
        useThreads=bThreads,
    )
    t1 = time.time()
    dt0 = LoggingWork(logger, cnt, bWithException, message)
    logger.shutdown()
    dt = time.time() - t1
    print(f"  total: {dt0: .3f} {dt: .3f}")
    return dt


# noinspection PyShadowingNames
def DoFastLoggingRsDefault(
    cnt: int,
    level: int,
    pathName: str | None,
    bRotate: bool,
    bWithException: bool,
    message: str,
) -> float:
    if bRotate:
        size = MB
        count = 8
    else:
        size = 0
        count = 0
    fl.set_console_writer()
    fl.set_file_writer(level, pathName, size, count)
    t1 = time.time()
    dt0 = LoggingWork(fl, cnt, bWithException, message)
    fl.sync(1.0)
    dt = time.time() - t1
    print(f"  total: {dt0: .3f} {dt: .3f}")
    return dt


# noinspection PyShadowingNames
def DoFastLoggingRs(
    cnt: int,
    level: int,
    pathName: str | None,
    bRotate: bool,
    bWithException: bool,
    message: str,
) -> float:
    if bRotate:
        size = MB
        count = 8
    else:
        size = 0
        count = 0
    logger = Logging(
        level,
        file=pathName,
        max_size=size,
        backlog=count,
    )
    t1 = time.time()
    dt0 = LoggingWork(logger, cnt, bWithException, message)
    logger.shutdown()
    dt = time.time() - t1
    print(f"  total: {dt0: .3f} {dt: .3f}")
    return dt


def Measure(
    num: int,
    prefix: str,
    cbFunc: callable,
    cnt: int,
    level: int,
    fileName: str,
    bRotate: bool,
    bWithException: bool,
    msg: str,
    message: str,
    *args,
) -> float:
    title = GetTitle(prefix, msg, fileName, bRotate, bWithException, level)
    print(f"{num} {prefix}: {title}")
    dt = 0
    for i in range(10):
        pathName = GetPathName(tmpDirName, fileName, title)
        dt += cbFunc(cnt, level, pathName, bRotate, bWithException, message, *args)
        if dt > 2.0:
            break
    dt /= i + 1
    return dt


if __name__ == "__main__":
    num = 0
    cnt = 5000
    print("cnt:", cnt)
    fileName = "logging.log"
    fastFileName = "logging.log"
    htmlTemplate = open("doc/benchmarks/template.html").read()
    dtAllJson = {}
    for msg, message in (
        # ("short", "Message"),
        (
            "long",
            "Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message",
        ),
    ):
        dtAllJsonMsg = dtAllJson[msg] = {}
        for bWithException in (False, True):
            dtAllJsonMsgExc = dtAllJsonMsg["exc" if bWithException else "noexc"] = {}
            for title, name, fileName, bRotate in (
                # ("No log file", "nolog", None, False),
                # ("Log file", "file", "logging.log", False),
                ("Rotating log file", "rotate", "logging.log", True),
            ):
                dtAllJsonMsgExcName = dtAllJsonMsgExc[name] = {}
                dtAll = {"TITLE": title}
                for level in (DEBUG, INFO, WARNING, ERROR, CRITICAL):
                    for _ in range(10):
                        Measure(
                            num,
                            "FastLoggingRsDefault",
                            DoFastLoggingRsDefault,
                            cnt,
                            level,
                            fileName,
                            bRotate,
                            bWithException,
                            msg,
                            message,
                        )
                    dts = [
                        Measure(
                            num,
                            "Logging",
                            DoLogging,
                            cnt,
                            level,
                            fileName,
                            bRotate,
                            bWithException,
                            msg,
                            message,
                        ),
                        Measure(
                            num,
                            "FastLogging",
                            DoFastLogging,
                            cnt,
                            level,
                            fileName,
                            bRotate,
                            bWithException,
                            msg,
                            message,
                            False,
                        ),
                        Measure(
                            num,
                            "FastLoggingThread",
                            DoFastLogging,
                            cnt,
                            level,
                            fileName,
                            bRotate,
                            bWithException,
                            msg,
                            message,
                            True,
                        ),
                        Measure(
                            num,
                            "FastLoggingRs",
                            DoFastLoggingRs,
                            cnt,
                            level,
                            fileName,
                            bRotate,
                            bWithException,
                            msg,
                            message,
                        ),
                        Measure(
                            num,
                            "FastLoggingRsDefault",
                            DoFastLoggingRsDefault,
                            cnt,
                            level,
                            fileName,
                            bRotate,
                            bWithException,
                            msg,
                            message,
                        ),
                    ]
                    dtAllJsonMsgExcName[Level2Sym(level).name] = {
                        "logging": dts[0],
                        "fastlogging": dts[1],
                        "fastlogging-threads": dts[2],
                        "fastlogging-rs": dts[3],
                        "fastlogging-rs-default": dts[4],
                    }
                    dtAll[Level2Sym(level).name] = ", ".join(
                        [f"{dt: .4f}" for dt in dts]
                    )
                    num += 1
                if bWithException:
                    name += "_exc"
                with open(f"doc/benchmarks/{name}_{msg}.html", "w") as F:
                    F.write(htmlTemplate % dtAll)
    with open(f"doc/benchmarks/pybenchmarks.json", "w") as F:
        F.write(json.dumps(dtAllJson, indent=4))
