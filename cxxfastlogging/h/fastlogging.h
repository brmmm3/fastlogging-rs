#pragma once

#include <memory>
#include "rust/cxx.h"

namespace org
{
    namespace fastlogging
    {

        class FastLogger
        {
        public:
            FastLogger();
            ~FastLogger();

            void trace(rust::Str message) const;
            void debug(rust::Str message) const;
            void info(rust::Str message) const;
            void success(rust::Str message) const;
            void warning(rust::Str message) const;
            void error(rust::Str message) const;
            void critical(rust::Str message) const;
            void fatal(rust::Str message) const;
            void exception(rust::Str message) const;
        };

        std::unique_ptr<FastLogger> new_fastlogger();

    } // namespace fastlogging
} // namespace org
