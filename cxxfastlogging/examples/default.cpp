#include "cxxfastlogging/h/fastlogging.h"

int main()
{
    auto log = Logging::new_default();
    log->trace("Trace Message");
    log->debug("Debug Message");
    log->info("Info Message");
    log->success("Success Message");
    log->warning("Warning Message");
    log->error("Error Message");
    log->fatal("Fatal Message");
    log->shutdown(false);
    return 0;
}
