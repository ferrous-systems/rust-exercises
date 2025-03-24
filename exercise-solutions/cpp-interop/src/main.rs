use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxString};
use ffi::rapidcsv::Document;

autocxx::include_cpp! {
    #include "wrapper.h"
    generate!("rapidcsv::Document")
    generate!("my_csv::open_csv")
    generate!("my_csv::get_string_cell")
    safety!(unsafe_ffi)
}

fn main() {
    let_cxx_string!(file_name = "weather.csv");
    let doc = ffi::my_csv::open_csv(&file_name).within_unique_ptr();
    let count = doc.GetRowCount();
    let mut june_temps = 0.0;
    for i in 0..count {
        let date = doc.get_string_cell(0, i);
        // Convert to Rust str - with type-guaranteed no additional memory overhead
        if let Ok(date_str) = date.to_str() {
            // Check if it's in June - format MM/DD/YYY
            if date_str.starts_with("6/") {
                // Date is in June, so get the Temp_C value in the 2nd column
                if let Ok(temp) = doc.get_string_cell(1, i).to_str() {
                    if let Ok(temp_float) = temp.parse::<f64>() {
                        june_temps += temp_float;
                    }
                }
            }
        }
    }
    // June always has 30 days
    println!("{}", june_temps / 30.0);
}

trait GetStringCell {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString>;
}

impl GetStringCell for Document {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString> {
        ffi::my_csv::get_string_cell(self, column, row)
    }
}
