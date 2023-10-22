extern crate polars;
extern crate csv;
extern crate sys_info;

use polars::prelude::*;
use std::fs::File;
use csv::Reader;
use std::time::Instant;
use std::io::{self, Result as IoResult};
use std::num::ParseFloatError;

fn main() {
    match run_program() {
        Ok(1) => println!("Program executed successfully!"),
        Err(e) => eprintln!("Error occurred: {}", e),
        _ => eprintln!("Unknown error occurred."),
    }
}

pub fn run_program() -> IoResult<i32> {
    let start = Instant::now();
    let mem_info_before = sys_info::mem_info().unwrap();

    let results = describe_with_polars("SPX.csv")?;
    println!("{:?}", results);

    let number_of_observations = count_observations()?;
    println!("There are {} days observed in the dataset.", number_of_observations);

    let total = sum_volume()?;
    println!("Total of all volume traded: {}", total);
    
    let elapsed = start.elapsed();
    let mem_info_after = sys_info::mem_info().unwrap();
    let mem_used = mem_info_after.total - mem_info_before.total;

    println!("This took {:.2?} seconds to complete", elapsed);
    println!("This used {} MB of memory to complete", mem_used / 1024); // Convert KB to MB

    Ok(1)
}

fn describe_with_polars(file_path: &str) -> IoResult<DataFrame> {
    let df = CsvReader::from_path(file_path)
        .map_err(|e: PolarsError| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .infer_schema(None)
        .has_header(true)
        .finish()
        .map_err(|e: PolarsError| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(df.describe())
}

fn count_observations() -> IoResult<usize> {
    let file = File::open("SPX.csv")?;
    let mut rdr = Reader::from_reader(file);
    Ok(rdr.records().count())
}

fn sum_volume() -> IoResult<f64> {
    let file = File::open("SPX.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut total = 0.0;

    for result in rdr.records() {
        let record = result?;
        let value: f64 = record[6].parse()
            .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        total += value;
    }

    Ok(total)
}

