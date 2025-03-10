#![allow(unused_variables)] // Delete me once you start
#![allow(unused_imports)] // Delete me once you start

use autocxx::prelude::*;
use cxx::let_cxx_string;

use autocxx::prelude::*;
autocxx::include_cpp! {
    #include "rapidcsv.h"
    generate!("rapidcsv::Document")
    safety!(unsafe_ffi)
}

//use crate::ffi::rapidcsv::Document;

fn main() {
    let_cxx_string!(file_name = "example.csv");
    // The below lines do **not** work :(
    //let doc = Document::new(&file_name).within_unique_ptr();
    //println!("{}", doc.GetRowCount());
}
