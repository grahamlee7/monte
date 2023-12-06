
// main.rs provides random functions in monte
// ver 06 Dec 23
   use crate::monty::monty2;
 //  use std::env;
   pub mod monty;

use std::env;   
extern crate rand;
// import commonly used items from the prelude:
use rand::prelude::*;
use indicatif::ProgressBar;

fn main () {  // random	
	let args: Vec<_> = env::args().collect();
	if args.len() == 1 {	// no args option
        println!("Usage: random option");
		println!("Examples: random pi or random dice ");
		println!("For help: random --help ");
		println!("For version: random --V ");
    } else {  // with args option
  	    let arg1 = env::args().nth(1).expect("no arg1");
//            println!("arg1={}", arg1);
	    if arg1 == "pi"  {  // finding pi
			let mut ctr = 1; // loop ctr
			let mut ctr2 =1.0; // divisor
			if args.len() == 2 { // no count, default to 4
			    let arg2 = "4";
				ctr = arg2.parse().expect("not an int");
				ctr2 = arg2.parse().expect("not a float");
				println!("no count specified, so using {ctr} as default");
				println!("a higher count is more accurate but takes longer");
			}  // end 2 args
			if args.len() == 3 {	// with count
				let arg2 = env::args().nth(2).expect("no arg2");
				ctr = arg2.parse().expect("not an int");
				ctr2 = arg2.parse().expect("not a float");
				if ctr > 10 {	// too big
				    println!("count {ctr} too big for pi, using 10");
					ctr=10; ctr2=10.0;
				}	
			}  // end 3 args 
		println!("Finding π using {ctr} iterations");
	        let mut i = 0;
	        let mut pivalue = 0.0;
			let pb = ProgressBar::new(ctr);
            while i < ctr {   // cycle thru monty2
 	            pivalue = pivalue + monty2();
				pb.inc(1);
                i = i + 1;
	        }   // end while
//	        println!("pisum = {:.4}", pivalue);
            pivalue = (pivalue)/ctr2;
	        println!("Averaged pivalue = {:.4}" , pivalue);	 
		} else if arg1 == "dice" {   // roll dice
			let mut ctr = 1; // throw ctr
			if args.len() == 2 { // no count, default to 6 
				ctr = 6;
			    println!("no throw count specified, so using 6 as default");
				}  // end 2 args
			if args.len() == 3 {	// with count
				let arg2 = env::args().nth(2).expect("no arg2");
				ctr = arg2.parse().expect("not an int");
				if ctr > 12 {	// too big
				    println!("count {ctr} too big for the screen, using 12");
					ctr=12; 
				}	
			}  // end 3 args
			println!("Dice option: 2 dies, {ctr} throws");
			let mut i = 0;
			let mut rng = thread_rng();
			print!("Dice roll:" );
			while i < ctr {   // throw die
				let first = rng.gen_range(1..=6);
				print!("{}", first);
				let second = rng.gen_range(1..=6);
				print!("+{} ", second);
				if first == 6 && second == 6 { // win
				print!("Win! "); 
				print! {"{}",'\x07'}; break; // bell and that's it
				}
				if first == 1 && second == 1 { // lose
				   print!("Lose! ");  
				   print! {"{}",'\x07'}; break; // bell and that's it
				}
				i = i + 1;
			}   // end while i	
			println!();
		} else if arg1.contains("help") || arg1.contains("-h") {   // help, -help, --help or -h
		    println!("This program demonstrates random maths");
			println!("Using discrete maths and 64 bit integers");
			println!("Usage: random option count");
			println!("Example 1: random pi does a monte carlo simulation a number of times ");
			println!("Example 2: random dice throws some dice a number of times ");
		} else if arg1 == "--V" {   // version
			println!("Version 1.03 06 Dec 23");
		} else {  // not pi, dice or help
            println!("unsuported random option {arg1}");
		    println!("this version only supports Pi or Dice options");
	    }   // end help
	}	// end args
}  // end main
