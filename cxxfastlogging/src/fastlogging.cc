#include "cxxfastlogging/h/fastlogging.h"
#include <iostream>
#include <memory>

namespace org
{
    namespace fastlogging
    {

        class FastLoggerImpl
        {
        public:
            void trace(const std::string &message) { std::cout << "[TRACE] " << message << std::endl; }
            void debug(const std::string &message) { std::cout << "[DEBUG] " << message << std::endl; }
            void info(const std::string &message) { std::cout << "[INFO] " << message << std::endl; }
            void success(const std::string &message) { std::cout << "[SUCCESS] " << message << std::endl; }
            void warning(const std::string &message) { std::cout << "[WARNING] " << message << std::endl; }
            void error(const std::string &message) { std::cout << "[ERROR] " << message << std::endl; }
            void critical(const std::string &message) { std::cout << "[CRITICAL] " << message << std::endl; }
            void fatal(const std::string &message) { std::cout << "[FATAL] " << message << std::endl; }
            void exception(const std::string &message) { std::cout << "[EXCEPTION] " << message << std::endl; }
        };

        class FastLogger
        {
        public:
            FastLogger() : impl_(std::make_unique<FastLoggerImpl>()) {}
            ~FastLogger() = default;

            void trace(rust::Str message) const { impl_->trace(std::string(message)); }
            void debug(rust::Str message) const { impl_->debug(std::string(message)); }
            void info(rust::Str message) const { impl_->info(std::string(message)); }
            void success(rust::Str message) const { impl_->success(std::string(message)); }
            void warning(rust::Str message) const { impl_->warning(std::string(message)); }
            void error(rust::Str message) const { impl_->error(std::string(message)); }
            void critical(rust::Str message) const { impl_->critical(std::string(message)); }
            void fatal(rust::Str message) const { impl_->fatal(std::string(message)); }
            void exception(rust::Str message) const { impl_->exception(std::string(message)); }

        private:
            std::unique_ptr<FastLoggerImpl> impl_;
        };

        std::unique_ptr<FastLogger> new_fastlogger()
        {
            return std::make_unique<FastLogger>();
        }

    } // namespace fastlogging
} // namespace org
