use rand::{Rng, thread_rng};
use std::env;


fn main() {
    // declare initial variables
    let mut pw_chars: String = String::from("\
    abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    0123456789\
    !@#$%^&*(){}|:<>?,./;[]-=_+");

    let help: &str = "usage: genpass [--L length of password | --E characters to exclude]";

    // arguments -> vector
    let args: Vec<String> = env::args().collect();

    let mut length: i32 = 16;

    let mut rng_seed = thread_rng();

    match args.len() {
        1 => {
            // do nothing, program will move to initial state
        },
        2..=5 => {
            // loop through arguments
            for (i,arg) in args[1..].iter().enumerate() {
                // zeroeth index (i) is the the argument right after ./binary
                // if match each argument to see if it applies to length or excluding
                match arg.as_str() {
                    "--L" | "-length" => {
                        if i+1 > args.len()-1 {
                            eprintln!("Length should be after flag!");
                            eprintln!("{}",help);
                        }
                        length = args[i+2].parse::<i32>().unwrap();
                    },
                    "-exclude" | "--E" => {
                        if i+1 > args.len() {
                            eprintln!("exclude characters should be after flag!");
                            eprintln!("{}",help);
                        }
                        for i in args[i+2].chars() {
                            pw_chars = pw_chars.replace(i,"");
                        }
                    },
                    "-help" | "--H" | "--help" => {
                        eprintln!("{}",help);
                    },
                    _ => continue,
                }
            }
        }
        _ => {
            eprintln!("Not correct number of arguments!");
            eprintln!("{}",help);
        }
    }
    // do the random stuff here
    for _ in 0..=length {
        //pass
        let index = rng_seed.gen_range(0..pw_chars.len());
        print!("{}", pw_chars.as_bytes()[index] as char);
    }
    println!();
}