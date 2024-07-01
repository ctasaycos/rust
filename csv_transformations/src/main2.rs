use std::error::Error;
use std::fs::File;
use csv::WriterBuilder;

#[derive(Debug, serde::Serialize)]
struct Record {
    id: u32,
    name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create some example data
    let data = vec![
        Record { id: 1, name: "Alice".to_string(), age: 30 },
        Record { id: 2, name: "Bob".to_string(), age: 25 },
        Record { id: 3, name: "Charlie".to_string(), age: 35 },
    ];

    // Specify the output CSV file path
    let output_path = "dataset.csv";

    // Write the data to a CSV file
    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .from_writer(File::create(output_path)?);

    for record in &data {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    println!("CSV file '{}' successfully created!", output_path);
    Ok(())
}
