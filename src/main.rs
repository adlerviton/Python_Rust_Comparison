extern crate polars;

use polars::prelude::*;
use std::fs::File;
use csv::Reader;
use std::error::Error;
use std::time::Instant;

fn main() {
    match run_program() {
        Ok(1) => println!("Program executed successfully!"),
        Err(e) => eprintln!("Error occurred: {}", e),
        _ => eprintln!("Unknown error occurred."),
    }
}

pub fn run_program() -> Result<i32, Box<dyn Error>> {
    let start = Instant::now();
    let mem_info_before = sys_info::mem_info().unwrap();

    let results = describe_with_polars("SPX.csv")?;
    println!("{:?}", results);

    let number_of_observations = count_observations()?;
    println!("There are {} days observed in the dataset.", number_of_houses);

    let total = sum_volume()?;
    println!("Total of all volume traded: {}", total);
    
    let elapsed = start.elapsed();
    let mem_info_after = sys_info::mem_info().unwrap();
    let mem_used = mem_info_after.total - mem_info_before.total;


    println!("this took {} seconds to complete", elapsed);
    println!("this used {} MB of memory to complete", mem_used);
    std::fs::write(
        "rust_performance.md",
        format!(
            "## Rust Performance Report\n- Time taken: {:.2?} seconds\n- Memory used: {} KB\n### Operations Performed\n1. Read CSV file\n2. Count number of observations\n3. Sum median house values\n", 
            elapsed, mem_used
        )
    )?;

    Ok(1)
}

fn describe_with_polars(file_path: &str) -> Result<DataFrame> {
    // Read the CSV file into a DataFrame
    let df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Call the describe method on the DataFrame
    df.describe()
}

fn count_observations() -> Result<usize, Box<dyn Error>> {
    let file = File::open("SPX.csv")?;
    let mut rdr = Reader::from_reader(file);
    Ok(rdr.records().count())
}

fn sum_volume() -> Result<f64, Box<dyn Error>> {
    let file = File::open("SPX.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut total = 0.0;

    for result in rdr.records() {
        let record = result?;
        let value: f64 = record[6].parse()?;
        total += value;
    }

    Ok(total)
}