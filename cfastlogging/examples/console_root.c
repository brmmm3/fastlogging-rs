#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

// File: console_root.c
//
// Sample library usage.
int main(void)
{
    root_init();
    root_trace("Trace Message");
    root_debug("Debug Message");
    root_info("Info Message");
    root_success("Success Message");
    root_warning("Warning Message");
    root_error("Error Message");
    root_fatal("Fatal Message");
    root_shutdown(0);
    return 0;
}
