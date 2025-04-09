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

    let sum_of_june_temperatures: i64 = (0..count)
        .filter_map(|row_index| {
            let date: UniquePtr<CxxString> = doc.get_string_cell(0, row_index);
            let date: &str = date.to_str().ok()?;
            // Check if it's in June: the date format in the file is `M/DD/YYYY`
            date.starts_with("6/").then_some(row_index)
        })
        .filter_map(|row_index| {
            let temperature: UniquePtr<CxxString> = doc.get_string_cell(1, row_index);
            let temperature: &str = temperature.to_str().ok()?;
            temperature.parse::<i64>().ok()
        })
        .sum();
    // June has 30 days
    println!("{:.3}", sum_of_june_temperatures as f64 / 30.0);
}

trait GetStringCell {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString>;
}

impl GetStringCell for Document {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString> {
        ffi::my_csv::get_string_cell(self, column, row)
    }
}
