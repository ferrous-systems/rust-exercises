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
    let_cxx_string!(file_name = "example.csv");
    let doc = ffi::my_csv::open_csv(&file_name).within_unique_ptr();
    let count = doc.GetRowCount();
    for i in 0..count {
        let date = doc.get_string_cell(0, i);
        println!("{}", date);
    }
}

trait GetStringCell {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString>;
}

impl GetStringCell for Document {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString> {
        ffi::my_csv::get_string_cell(self, column, row)
    }
}

