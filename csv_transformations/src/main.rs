mod main2;

use std::error::Error;
use std::fs::File;
use std::io::{Write};
use csv::{ReaderBuilder, WriterBuilder};

fn read_and_modify_csv(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(file);

    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .from_writer(File::create(output_path)?);

    // Read the headers and modify them
    let headers = rdr.headers()?.clone();
    let modified_headers: Vec<String> = headers.iter()
        .map(|h| if h == "city" { "location".to_string() } else { h.to_string() })
        .collect();  

    wtr.write_record(&modified_headers)?;

    // Write the remaining records
    for result in rdr.records() {
        let record = result?;
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "testing.csv"; // specify your input CSV file path here
    let output_path = "modified_testing3.csv"; // specify your output CSV file path here
    read_and_modify_csv(input_path, output_path)?;
    Ok(())
}