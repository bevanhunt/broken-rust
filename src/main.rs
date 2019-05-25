extern crate calamine;
extern crate chrono;
extern crate crossbeam;
extern crate walkdir;

use calamine::open_workbook;
use calamine::Xlsx;
use chrono::prelude::Local;
use crate::calamine::Reader;
use walkdir::WalkDir;
use crossbeam::sync::WaitGroup;
use std::thread;

fn main() {
    let start = Local::now();
    let wg = WaitGroup::new();
    for entry in WalkDir::new("../Downloads/binance") {  
        let wg = wg.clone();
        thread::spawn(move || {
            let entry = entry.unwrap();
            if entry.path().is_file() && entry.path().extension().unwrap() == "xlsx" {
                let mut excel: Xlsx<_> = open_workbook(entry.path()).unwrap();
                if let Some(Ok(r)) = excel.worksheet_range("sheet1") {
                    let x = r.rows().len();
                    println!("rows {}", x);
                }
            }
            drop(wg);
        });
    }
    wg.wait();
    let end = Local::now();
    let foo = end-start;
    println!("time {}", foo.num_milliseconds());
}
