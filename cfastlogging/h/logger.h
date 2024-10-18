#ifndef CFASTLOGGING_LOGGER_H
#define CFASTLOGGING_LOGGER_H

Logger logger_new(uint8_t level, const char *domain);

Logger logger_new_ext(uint8_t level, const char *domain, int8_t tname, int8_t tid);

void logger_set_level(Logger logger, uint8_t level);

void logger_set_domain(Logger logger, const char *domain);

// Logger calls

int logger_trace(Logger logger, const char *message);

int logger_debug(Logger logger, const char *message);

int logger_info(Logger logger, const char *message);

int logger_success(Logger logger, const char *message);

int logger_warning(Logger logger, const char *message);

int logger_error(Logger logger, const char *message);

int logger_critical(Logger logger, const char *message);

int logger_fatal(Logger logger, const char *message);

int logger_exception(Logger logger, const char *message);

#endif
