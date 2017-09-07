extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::error::Error;
use std::process;
use std::collections::HashMap;
use structopt::StructOpt;

/// Count field pairs
#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
struct Cli {
    /// Input filename
    filename: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    a_field: String,
    b_field: String,
}

fn count(filename: String) -> Result<(), Box<Error>> {
    let mut field_pairs = HashMap::new();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(filename)
        .unwrap();

    for result in reader.deserialize() {
        let record: Record = result?;
        let pair = format!("{}-{}", record.a_field, record.b_field);
        let count = field_pairs.entry(pair).or_insert(0);
        *count += 1;
    }

    let mut counts = HashMap::new();

    let mut keys = vec![];
    for (pair, count) in &field_pairs {
        let pairs = counts.entry(count).or_insert(vec![]);
        pairs.push(pair);
        keys.push(count)
    }

    keys.sort();
    keys.dedup();
    keys.reverse();

    for key in keys {
        println!("{}:\t{:?}", key, counts.get(key).unwrap());
    }

    Ok(())
}

fn main() {
    let args = Cli::from_args();
    if let Err(err) = count(args.filename) {
        println!("error counting stats: {}", err);
        process::exit(1);
    }
}
