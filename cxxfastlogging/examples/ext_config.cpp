#include "cxxfastlogging/h/fastlogging.h"

int main()
{
    auto log = Logging::create(DEBUG, "root", {});
    log->set_ext_config(ExtConfigFfi{
        MessageStructEnum::Xml,
        true,
        false,
        true,
        false,
        true,
    });
    log->info("Structured XML message");
    log->shutdown(false);
    return 0;
}
