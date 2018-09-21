use clap::Arg;

/// Clap validator testing if a string can be a valid usize.
pub fn cli_validator_usize(v: String)->Result<(), String>{
    match usize::from_str_radix(&v, 10){
        Ok(_) => Ok(()),
        Err(s) => Err(s.to_string())
    }
}

/// Create a ngram clap argument
pub fn clap_arg_ngram()->Arg<'static, 'static>{
    Arg::with_name("ngram")
        .short("n")
        .long("ngram")
        .takes_value(true)
        .default_value("1")
        .validator(cli_validator_usize)
        .help("Generate ngram frequency from 1 to ngram")
}
