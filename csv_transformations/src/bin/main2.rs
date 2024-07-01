use std::error::Error;
use std::fs::File;
use csv::WriterBuilder;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Record {
    id: u32,
    name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = vec![
        Record { id: 1, name: "Alice".to_string(), age: 30 },
        Record { id: 2, name: "Bob".to_string(), age: 25 },
        Record { id: 3, name: "Charlie".to_string(), age: 35 },
        Record { id: 4, name: "Samantha".to_string(), age: 27 },
    ];

    let output_path = "dataset.csv";

    let writer_builder = &mut WriterBuilder::new();
    let mut wtr = writer_builder
        .delimiter(b',')
        .from_writer(File::create(output_path)?);

    for record in &data {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    println!("CSV file '{}' successfully created!", output_path);
    Ok(())
}
