use std::str;

use clap::{Parser, Subcommand};
use rusty_leveldb::{LdbIterator, Options};

#[derive(Debug, Parser)]
#[clap(name = "git")]
#[clap(about = "A fictional versioning CLI", long_about = None)]
struct CliArgs {
    /// The database file path
    db: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Get { key: String },
    Put { key: String, value: String },
    Delete { key: String },
    List,
}

fn main() {
    let args = CliArgs::parse();

    let options = Options {
        create_if_missing: true,
        ..Default::default()
    };
    let mut database = rusty_leveldb::DB::open(&args.db, options)
        .unwrap_or_else(|e| panic!("failed to open database: {:?}", e));

    match args.command {
        Command::Get { key } => {
            let key_bytes = key.as_bytes();
            let data = database.get(key_bytes);
            print_kv(key_bytes, data.as_deref());
        }
        Command::Put { key, value } => {
            database
                .put(key.as_bytes(), value.as_bytes())
                .unwrap_or_else(|e| panic!("failed to write to database: {:?}", e));
        }
        Command::Delete { key } => {
            database
                .delete(key.as_bytes())
                .unwrap_or_else(|e| panic!("failed to delete from database: {:?}", e));
        }
        Command::List => {
            let mut iter = database
                .new_iter()
                .unwrap_or_else(|e| panic!("failed to open iterator to database: {:?}", e));
            while let Some((key, value)) = iter.next() {
                print_kv(key.as_slice(), Some(&value));
            }
        }
    }
}

fn print_kv(key: &[u8], value: Option<&[u8]>) {
    fn to_string(v: &[u8]) -> String {
        str::from_utf8(v)
            .map(|s| s.to_owned())
            .unwrap_or_else(|_e| format!("{:?}", v))
    }

    if let Some(value) = value {
        println!("{}={}", to_string(key), to_string(value));
    } else {
        println!("{} is absent", to_string(key));
    }
}
