extern crate clap;

use clap::{Arg, Command};

#[derive(Debug)]
pub struct CommandArgs  {
    pub filename: String,
    pub start: i64,
    pub end: i64,
    pub option1: bool,
}

impl CommandArgs  {
    pub fn new() -> Self {
        // basic app information
        let app = Command::new("sum2")
            .version("1.0")
            .about("Caculates sum2 for input file")
            .author("Marvin Mednick");

        // Define the name command line option
        let filename_option = Arg::new("file")
            .takes_value(true)
            .help("Input file name")
            .required(true);

        // Define the starting value option
        let start_option = Arg::new("start")
            .takes_value(true)
            .allow_hyphen_values(true)
            .required(true)
            .help("lowest value for t");

        // Define the starting value option
        let end_option = Arg::new("end")
            .takes_value(true)
            .required(true)
            .allow_hyphen_values(true)
            .help("highest value for t");

        // Define the starting value option
        let opt1_option = Arg::new("opt1")
            .long("opt1")
            .takes_value(false)
            .help("use option 1 sum -- hashset lookup");

        // now add in the argument we want to parse
        let mut app = app.arg(filename_option);
        app = app.arg(start_option);
        app = app.arg(end_option);
        app = app.arg(opt1_option);

        // extract the matches
        let matches = app.get_matches();

        // Extract the actual name
        let filename = matches.value_of("file")
            .expect("Filename can't be None, we said it was required");

        let start_str = matches.value_of("start");

        let start = match start_str {
            None => { println!("Start is None..."); 0},
            Some(s) => {
                match s.parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => {println!("That's not a number! {}", s); 0},
                }
            }
        };
        let end_str = matches.value_of("end");

        let end = match end_str {
            None => { println!("End is None..."); 0},
            Some(s) => {
                match s.parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => {println!("That's not a number! {}", s); 0},
                }
            }
        };
        println!("clap args: {} {} {} {}",filename,start,end,matches.is_present("opt1")); 

        CommandArgs { filename: filename.to_string(), start: start, end: end ,option1: matches.is_present("opt1")}
    }   
}
