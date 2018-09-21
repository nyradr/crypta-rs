#[macro_use]
extern crate clap;
extern crate serde_json;

extern crate crypta_rs;

use std::fs::File;
use std::io::Read;

use clap::{App, Arg, ArgMatches};
use crypta_rs::utils::ngram::NgramCounter;
use crypta_rs::utils::cli::cli_validator_usize;

// Get the input text
fn get_text(args: &ArgMatches)->Vec<u8>{
    match args.value_of("text"){
        Some(text) => text.as_bytes().to_owned(),
        None => {
            match args.value_of("file"){
                Some(path) => {
                    let mut file = File::open(path).unwrap();
                    let mut text = vec!();
                    file.read_to_end(&mut text).unwrap();
                    text
                },
                None => vec!() // impossible case
            }
        }
    }
}

fn frequency(args: ArgMatches){
    let text = get_text(&args);
    let ngram = value_t!(args, "ngram", usize).unwrap();

    let counter = NgramCounter::from_bytes(&text, ngram);
    let res = counter.to_counterfrequency_json();

    match args.value_of("export"){
        Some("json") => {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        },
        _ => {}
    }

}

fn main(){
    let matches = App::new("Frequency")
        .version("1.0")
        .author("nyradr : nyradr@protonmail.com")
        .about("Text frequency analysis")
        .arg(
            Arg::with_name("file")
            .short("i")
            .long("file")
            .takes_value(true)
            .required(true)
            .conflicts_with("text")
            .help("Set the text file to analyse")
        )
        .arg(
            Arg::with_name("text")
            .short("t")
            .long("text")
            .takes_value(true)
            .required(true)
            .conflicts_with("file")
            .help("Set the text to analyse as the argument value")
        )
        .arg(
            Arg::with_name("ngram")
            .short("n")
            .long("ngram")
            .takes_value(true)
            .default_value("1")
            .validator(cli_validator_usize)
            .help("Split text in group of ngram characters and calculate the frequency of theses ngrams. ngram must be greater than 0.")
        ).arg(
            Arg::with_name("export")
            .short("e")
            .long("export")
            .takes_value(true)
            .default_value("json")
            .possible_values(&["json"])
            .help("Result export format (json: as a json object)")
        )
        .get_matches();

    frequency(matches);
}
