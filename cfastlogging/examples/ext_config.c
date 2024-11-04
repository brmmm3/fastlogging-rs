#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

// File: ext_config.c
//
// Sample library usage.
int main(void)
{
    CExtConfig *config = ext_config_new(MessageStructEnum_Xml, 1, 0, 1, 0, 1);
    printf("config.structured=%d\n", config->structured);
    printf("config.hostname=%d\n", config->hostname);
    printf("config.pname=%d\n", config->pname);
    printf("config.pid=%d\n", config->pid);
    printf("config.tname=%d\n", config->tname);
    printf("config.tid=%d\n", config->tid);
    printf("-------- Finished --------\n");
    return 0;
}
