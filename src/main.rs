use std::path::Path;
use std::fs::File;
use std::io::{self,prelude::*, BufReader};
use regex::Regex;
use std::process::exit;

mod cmd_line;
use crate::cmd_line::CommandArgs;
use std::collections::HashSet;
use std::collections::BTreeSet;
// extern crate time;

use std::time::Instant;


fn main() {


    let cmd_line = CommandArgs::new();

    println!("Hello, {:?}!",cmd_line);

    println!("Calulating sum2 for t from {} to {}",cmd_line.start,cmd_line.end);
  // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };


    let mut input_set = BTreeSet::<i64>::new();
    let mut target_set = HashSet::<i64>::new();
    let mut result_set = HashSet::<i64>::new();
    let mut skip_set = HashSet::<i64>::new();
    let before = Instant::now();
    for i in cmd_line.start..=cmd_line.end {
        target_set.insert(i as i64);
    }
    println!("Time to setup target {:.2?}",before.elapsed());

    let before = Instant::now();
    let reader = BufReader::new(file);
	let mut _count = 0;
	let mut _count_valid = 0;
	let mut _count_invalid = 0;
    for line in reader.lines() {
		_count += 1;	
        if _count % 100000 == 0 {
            println!("*")
        }
        else if _count % 1000 == 0 {
            print!(".")
        }
		let line_data = line.unwrap();
        let num = line_data.parse::<i64>().unwrap();
        input_set.insert(num);
        continue;

        // find the the input number, ignoring leading whitespace
        let re_input = Regex::new(r"^\s*(?P<number>-*\d+).*$").unwrap();

        if let Some(x) = re_input.captures(&line_data) {
               _count_valid += 1;	
               let match_numstr = x.name("number").map_or("", |m| m.as_str());
//               println!("match is {}",match_numstr);
               let num = match_numstr.parse::<i64>().unwrap();
               //println!("Found {}",num);
//               input_set.insert(num);


        }
        else {
           _count_invalid += 1;	
            println!("Skipping line #{} {}",_count,line_data)
        };

    }
    println!("Time to read {:.2?}",before.elapsed());
    //exit(0);

    println!("Input read {} lines {} valid, {} invalid",_count,_count_valid,_count_invalid);
    println!("Resulting in  {} unique entries",input_set.len());
    println!("Target size is {} ",target_set.len());

//    println!("Input {:?} Target {:?}",input_set,target_set);

    let before = Instant::now();

    _count = 0;
    if cmd_line.option1 {
        for x in &input_set {
            _count += 1;	
            if _count % 100000 == 0 {
                println!("*")
            }
            else if _count % 1000 == 0 {
                print!(".");
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
    //        println!("Looking for {}",t);
            for t in &target_set {
                if skip_set.contains(&t) {
                    continue;
                }
               // println!("Checking {}",t);
                let target = t.clone();
                let y = target - x;
                if input_set.contains(&y) {
                //    println!("Found {}",target);
                    result_set.insert(target.clone());
                    // remoue t from the target set so we don't look for it any more
                    skip_set.insert(target);
                    //println!("Input {:?} Target {:?}",input_set,target_set);
                }

            }

        }

    } else {
        for x in &input_set {
            _count += 1;	
            if _count % 100000 == 0 {
                println!("*")
            }
            else if _count % 1000 == 0 {
                print!(".");
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
            let search_range = cmd_line.start-x..=cmd_line.end-x;
                for y in input_set.range(search_range) {
                    result_set.insert(x+y);
                }
        }       

    }
    println!("Time to sum {:.2?}",before.elapsed());
    println!("Output: Result size {}",result_set.len());
 //   println!("Result {:?}",result_set);

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */
/*
// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

	fn setup_basic1() -> Graph {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.add_edge(2,4),Some(2));
		assert_eq!(g.add_edge(3,4),Some(1));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_outgoing(1),&[2]);
		assert_eq!(g.get_incoming(2),&[1]);
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_incoming(2),&[1]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.add_edge(1,2),Some(3));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[3]);
		
	}


 }
 */
