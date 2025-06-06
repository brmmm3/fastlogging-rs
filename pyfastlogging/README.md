# `pyfastlogging`

## Building the wheels

This is the Python layer for [fastlogging](https://github.com/brmmm3/fastlogging-rs/tree/master/fastlogging). This package is for creating Python wheels.  
For simplicity `build_wheels.py` can be used to build the wheels. The Python script uses `pyenv` to choose different Python versions and `maturin` to buld the wheels.  
If you run the script without options it will build the Python module for the current used Python interpreter.  
When using the `--versions` option you can provide:

- a comma separated list of Python versions
- `*` to build Python modules for all versions installed by `pyenv`. Put it in quotation marks to avoid unexpected behavior.

**Note:**
As of now [manylinux](https://github.com/pypa/manylinux) wheels are failing to build with `cibuildwheel`, because the spec for libc 2.34 is still not released.

For class and enum definitions see [here](doc/DEF.md).  
For root logger see [here](doc/ROOT.md).  
For logging class see [here](doc/LOGGING.md).  
For logger class see [here](doc/LOGGER.md).  
For examples look [here](examples) and [here](doc/EXAMPLES.md).
