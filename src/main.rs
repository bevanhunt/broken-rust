extern crate calamine;
extern crate chrono;
extern crate crossbeam;
extern crate walkdir;

use calamine::open_workbook;
use calamine::Xlsx;
use chrono::prelude::Local;
use crate::calamine::Reader;
use walkdir::WalkDir;

fn main() {
    let start = Local::now();
    for entry in WalkDir::new("../Downloads/binance") {
        let entry = entry.unwrap();
        if entry.path().is_file() && entry.path().extension().unwrap() == "xlsx" {
            let mut excel: Xlsx<_> = open_workbook(entry.path()).unwrap();
            if let Some(Ok(r)) = excel.worksheet_range("sheet1") {
                let x = r.rows().len();
                let end = Local::now();
                let foo = start - end;
                println!("rows {} and time {}", x, foo.num_seconds());
            }
        }
    }
}
