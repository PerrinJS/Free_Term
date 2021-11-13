extern crate rustyline;
extern crate yaml_rust;

use std::process::Command;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parser;

// TODO: refactor the main loop into it's own module
// TODO: implement pipes

fn main() {
    // TODO: this is just ment as a test
    // the <()> means use no compleater
    let mut rl = Editor::<()>::new();
    loop {
        // TODO: make the prompt configurable
        // TODO: implement tab compleation
        // TODO: implement buildin's such as cd
        // get a new line with the prompt "$"
        let readline = rl.readline("$ ");
        match readline {
            //if the line is blank do nothing
            Ok(line) => {
                /* We could add history with
                 * rl.add_history_entry(line.as_str());
                 */
                let trimmed_line = line.trim();
                if trimmed_line != "" {
                    /* There doesn't seem to be a traditional fork and exec
                     * just a combined thing */
                    // Create a new command executor
                    let parsed_args = parser::Parser::new(trimmed_line);
                    let mut to_run = Command::new(parsed_args.get_command());
                    to_run.args(parsed_args.get_args());
                    match to_run.spawn() {
                        Ok(mut child) => {
                            /* if it ran wait for it to finish and print
                             * any errors */
                            if let Err(e) = child.wait() {
                                println!("A bad thing happend: {}", e)
                            }
                        }
                        Err(err) => println!("Could not execute: {}", err),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Some error {:?}", err);
                break;
            }
        }
    }
}
