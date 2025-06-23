#pragma once

#include "rapidcsv.h"

namespace my_csv {
    rapidcsv::Document open_csv(const std::string& pPath) {
        return rapidcsv::Document(pPath);
    }

    std::string get_string_cell(const rapidcsv::Document& doc,const size_t pColumnIdx, const size_t pRowIdx) {
        return doc.GetCell<std::string>(pColumnIdx, pRowIdx);
    }
}
