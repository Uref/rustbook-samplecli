use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No file is specified");
    }



    
}

// use Builder pattern (clap@3.1.6)

// use clap::{Command, Arg};
// let matches = Command::new("My RPN program")
//     .version("1.0.0")
//     .author("Yusuke")
//     .about("Super awesome RPN calculator")
//     .arg(
//         Arg::new("formula_file")
//         .help("Formulas written in RPN")
//         .value_name("FILE")
//         .index(1)
//         .required(false),            
//     )
//     .arg(
//         Arg::new("verbose")
//         .help("Sets the level of verbosity")
//         .short('v')
//         .long("verbose")
//         .required(false),
//     )
//     .get_matches();

// match matches.value_of("formula_file") {
//     Some(file) => println!("File specified: {}", file),
//     None => println!("No file specified"),
// }
// let verbose = matches.is_present("verbose");
// println!("Is verbosity specified?: {}", verbose);


// no use cretes

// use std::env;
// let args: Vec<String> = env::args().collect();
// println!("{:?}", args);
