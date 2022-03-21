use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace()
                                .rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x)
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

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
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }    
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_ok() {
//         assert_eq!(2 * 2, 4);
//     }
// }

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
