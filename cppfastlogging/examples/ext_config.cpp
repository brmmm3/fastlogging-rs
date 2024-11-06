#include "h/cppfastlogging.hpp"

using namespace logging;

// File: ext_config.c
//
// Sample library usage.
int main(void)
{
    ExtConfig *ext_config = new ExtConfig(MessageStruct::Xml, 1, 0, 1, 0, 1);
    printf("config.structured=%d\n", (int)ext_config->config->structured);
    printf("config.hostname=%d\n", ext_config->config->hostname);
    printf("config.pname=%d\n", ext_config->config->pname);
    printf("config.pid=%d\n", ext_config->config->pid);
    printf("config.tname=%d\n", ext_config->config->tname);
    printf("config.tid=%d\n", ext_config->config->tid);
    printf("-------- Finished --------\n");
    return 0;
}
