#pragma once

#include "rapidcsv.h"

namespace my_csv {
    rapidcsv::Document open_csv(const std::string& pPath) {
        return rapidcsv::Document(pPath);
    }
}