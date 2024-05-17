#include "cppfastlogging.hpp"

using namespace logging;

int main(void)
{
    Logging *logging = new Logging();

    logging->debug("Debug Message");
    logging->info("Info Message");
    logging->warn("Warning Message");
    logging->error("Error Message");
    delete logging;

    return 0;
}
