use std::process::Command;
// TODO: impement actual argument parsing

#[derive(Clone)]
enum CmdAtom<'a> {
    Command(&'a str),
    Arg(&'a str),
    Sym(&'a str),
}

pub struct Parser<'p> {
    atoms: Vec<CmdAtom<'p>>,
}

impl<'p> Parser<'p> {
    fn parse_atoms_rec(mut string_parts: std::str::Split<'p, &str>) -> Vec<CmdAtom<'p>> {
        match string_parts.next() {
            Some(element) => {
                let mut so_far = Parser::parse_atoms_rec(string_parts);
                if so_far.len() == 0 {
                    // This is one above the end
                    so_far.push(CmdAtom::Command(element));
                } else {
                    // if we previously had a symbol we don't convert to arg
                    if let Some(CmdAtom::Command(element)) = so_far.pop() {
                        so_far.push(CmdAtom::Arg(element));
                    }
                    /* TODO: this is just while we get the structure
                     * we still need to handle pipes */
                    so_far.push(CmdAtom::Command(element));
                }
                so_far
            },
            None => {
                // initialize the list
                Vec::new()
            }
        }
    }

    fn parse_atoms(string_parts: std::str::Split<'p, &str>) -> Vec<CmdAtom<'p>> {
        let mut parsed_atoms = Parser::parse_atoms_rec(string_parts);
        parsed_atoms.reverse();
        parsed_atoms
    }

    pub fn new(whole_command: &'p str) -> Self {
        let command_words = whole_command.trim().split::<&str>(" ");
        let parsed_atoms = Parser::parse_atoms(command_words);
        Parser {
            atoms: parsed_atoms,
        }
    }

    /* WARNING: THE FOLLOWING TWO FUNCTIONS ARE JUST HERE AS PLACE HOLDERS */

    pub fn get_command(&self) -> &str {
        if let CmdAtom::Command(ret) = self.atoms[0]{
            ret
        } else {panic!("There should only be command sections here.")}
    }

    pub fn get_args(&self) -> Vec<&'p str> {
        let convert_to_str = |arg: &CmdAtom<'p>| -> &str {
            if let CmdAtom::Arg(ret) = arg {
                ret
            } else {panic!("There should only be args atm.")}
        };

        let arg_atoms: Vec<&str>;
        if self.atoms.len() > 1 {
            let mut atoms = self.atoms.clone();
            atoms.remove(0);
            arg_atoms = atoms.iter().map(convert_to_str).collect();
        } else {
            arg_atoms = Vec::new()
        }
        arg_atoms
    }
}

pub struct CmdExecutor<'c> {
    parsed_line: &'c Parser<'c>
}

impl<'c> CmdExecutor<'c> {
    pub fn new(to_exec: &'c Parser<'c>) -> Self {
        CmdExecutor {
            parsed_line: to_exec,
        }
    }

    //TODO: move the error messaging out into to main loop
    //fn run(&self) -> Result<(), std::io::Error> {
    pub fn run(&self) {
        let mut to_run = Command::new(self.parsed_line.get_command());
        to_run.args(self.parsed_line.get_args());
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
