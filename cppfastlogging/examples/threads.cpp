#include "h/cppfastlogging.hpp"
#include <cstdio>
#include <pthread.h>

using namespace logging;

void *loggerThreadFun(void *vargp)
{
    Logger *logger = static_cast<Logger *>(vargp);
    logger->trace("Trace Message");
    logger->debug("Debug Message");
    logger->info("Info Message");
    logger->success("Success Message");
    logger->warning("Warning Message");
    logger->error("Error Message");
    logger->fatal("Fatal Message");
    return nullptr;
}

int main(void)
{
    pthread_t thread_id;
    ExtConfig ext_config(MessageStruct::String, 1, 1, 1, 1, 1);
    WriterConfig configs[] = { ConsoleWriterConfig(DEBUG, true) };
    Logging *logging = new Logging(DEBUG, "root", configs, &ext_config);
    Logger  *logger  = new Logger(DEBUG, "LoggerThread", 1, 1);
    logging->add_logger(logger);
    pthread_create(&thread_id, nullptr, loggerThreadFun, static_cast<void *>(logger));
    logging->trace("Trace Message");
    logging->debug("Debug Message");
    logging->info("Info Message");
    logging->success("Success Message");
    logging->warn("Warning Message");
    logging->error("Error Message");
    logging->fatal("Fatal Message");
    pthread_join(thread_id, nullptr);
    delete logging;
    delete logger;
    return 0;
}
