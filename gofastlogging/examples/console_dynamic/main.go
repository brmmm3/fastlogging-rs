package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../lib/cfastlogging.h"
*/
import "C"

func main() {
	logging := C.logging_init()
	C.logging_trace(logging, C.CString("Trace message"))
	C.logging_debug(logging, C.CString("Debug message"))
	C.logging_info(logging, C.CString("Info Message"))
	C.logging_warning(logging, C.CString("Warning Message"))
	C.logging_error(logging, C.CString("Error Message"))
	C.logging_fatal(logging, C.CString("Fatal Message"))
	C.logging_shutdown(logging, 0)
}
