use std::process::Command;
use crate::builtins;

pub trait AbstractCmd {
    /* FIXME: this is not the final form as we need to still be able to get the input and output so
     * it can be passed to piped functions. */
    fn run(&mut self) -> Result<(), std::io::Error>;
}

// The following are wrappers around std::process::Command and the builtins

impl AbstractCmd for Command {
    fn run(&mut self) -> Result<(), std::io::Error> {
        match self.spawn() {
            Ok(mut child) => {
                /* if it ran wait for it to finish and print
                    * any errors */
                if let Err(e) = child.wait() {
                    Err(e)
                } else {
                    Ok(())
                }
            },
            Err(a) => { Err(a) }
        }
    }
}

impl AbstractCmd for builtins::Builtin<'_> {
    fn run(&mut self) -> Result<(), std::io::Error>
    {
        let to_call = self.func;
        println!("{}", *to_call(&self.args));
        Ok(())
    }
}
