use std::env;//bring std module into scope with a use statement so we can use its args function 
use std::fs;
use std::process;
use minigrep::Config;

//It's best to separare functionality so each function is responsible for one task.
//It's best to group the configuration varibles into one structure to make their purpose clear.
//It's best to print the specified reason
//Having all the error-handling code in one place will also ensure that we're printing messages that will be meaningful to our end users.

//Separation of concerns for Binary Project
//--split your program into a main.rs a lib.rs and move your program's logic to lib.rs
//--as long as your command line parsing logic is small, it can remain in main.rs.
//--When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs


//The responsibilities that remain in the main function after this process should be limited to the following:
//--Calling the command line parsing logic with the argument valus
//--Setting up any other configuration
//--calling a run function in lib.rs
//--Handling the error if run return an error
fn main() {
    //let args: Vec<String> = env::args().collect();//collect the args
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);
    }
    // //or
    // minigrep::run(config);
    //cargo run to pome.txt > output.txt will print the output to the output.txt file.
}


