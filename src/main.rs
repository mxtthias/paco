extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::collections::HashMap;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc;
use structopt::StructOpt;

/// Count field pairs
#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
struct Cli {
    /// Input filename
    filename1: String,
    filename2: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    a_field: String,
    b_field: String,
}

fn count(filename: String) -> HashMap<String, u32> {
    let mut field_pairs = HashMap::new();

    let reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(filename);

    let mut reader = match reader {
        Ok(csv_reader) => csv_reader,
        Err(error) => panic!("Could not create csv reader: {:?}", error),
    };

    for result in reader.deserialize() {
        let record: Record = match result {
            Ok(r) => r,
            Err(error) => panic!("Could not deserialize record: {:?}", error),
        };
        let pair = format!("{}-{}", record.a_field, record.b_field);
        let count = field_pairs.entry(pair).or_insert(0);
        *count += 1;
    }

    return field_pairs;
}

fn main() {
    let args = Cli::from_args();
    let filename1 = args.filename1;
    let filename2 = args.filename2;

    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let result = count(filename1);
        tx1.send(result).unwrap();
    });

    thread::spawn(move || {
        let result = count(filename2);
        tx2.send(result).unwrap();
    });

    let mut pairs = HashMap::new();

    for received in rx {
        for (pair, count) in received {
            let counts = pairs.entry(pair).or_insert(0);
            *counts += count;
        }
    }

    let mut counts = HashMap::new();
    let mut keys = vec![];

    for (pair, count) in pairs {
        let pairs = counts.entry(count).or_insert(HashSet::new());
        pairs.insert(pair);
        keys.push(count)
    }

    keys.sort();
    keys.dedup();
    keys.reverse();

    for key in keys {
        println!("{}:\t{:?}", key, counts.get(&key).unwrap());
    }
}
