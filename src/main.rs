extern crate os_pipe;
extern crate rustyline;
extern crate yaml_rust;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use free_term;

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
            Ok(line) => {
                /* We could add history with
                 * rl.add_history_entry(line.as_str());
                 */
                let trimmed_line = line.trim();
                //if the line is blank do nothing
                if trimmed_line != "" {
                    /* There doesn't seem to be a traditional fork and exec
                     * just a combined thing */
                    // Create a new command executor
                    let parsed_args = free_term::Parser::new(trimmed_line);
                    let executor = free_term::CmdExecutor::new(&parsed_args);
                    executor.run();
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted!");
                break;
            }
            Err(ReadlineError::Eof) => {
                // TODO: make this configureable
                println!("good bye!");
                break;
            }
            Err(err) => {
                println!("Some error {:?}", err);
                break;
            }
        }
    }
}
