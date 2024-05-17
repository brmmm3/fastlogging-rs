#include <stdlib.h>
#include <stdio.h>
#include "cfastlogging.h"
#include <string.h>

// File: main.c
//
// Sample library usage.
int main(void)
{
    void *logging = logging_init();

    logging_debug(logging, "Debug Message");
    logging_info(logging, "Info Message");
    logging_warning(logging, "Warning Message");
    logging_error(logging, "Error Message");
    logging_fatal(logging, "Fatal Message");

    logging_shutdown(logging, 0);

    return 0;
}
