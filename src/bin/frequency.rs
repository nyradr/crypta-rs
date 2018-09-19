#[macro_use]
extern crate clap;

extern crate crypta_rs;

use std::fs::File;
use std::io::Read;

use clap::{App, Arg, ArgMatches};


/// Validate the string as a valid usize number
fn usize_validator(v: String)->Result<(), String>{
    match usize::from_str_radix(&v, 10){
        Ok(i) => {
            if i > 0{
                Ok(())
            }else{
                Err("The value must be greater than 0.".to_string())
            }
        },
        Err(s) => Err(s.to_string())
    }
}

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
            Arg::with_name("count")
            .short("c")
            .long("count")
            .help("Count the number of occurence of the each characters")
        )
        .arg(
            Arg::with_name("freq")
            .short("f")
            .long("frequency")
            .help("Get the frequency of each characters")
        )
        .arg(
            Arg::with_name("ngram")
            .short("n")
            .long("ngram")
            .takes_value(true)
            .default_value("1")
            .validator(usize_validator)
            .help("Split text in group of ngram characters and calculate the frequency of theses ngrams. ngram must be greater than 0.")
        ).arg(
            Arg::with_name("export")
            .short("e")
            .long("export")
            .takes_value(true)
            .default_value("json")
            .possible_values(&["json"])
            .help("Result export format (json: as a json object)")
        ).arg(
            Arg::with_name("length")
            .short("l")
            .long("length")
            .help("Include the total length of the text and the number of ngram referenced.")
        )
        .get_matches();

    frequency(matches);
}
