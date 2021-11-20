use os_pipe;
use std::process::Command;
// TODO: impement actual argument parsing

/* TODO:
 * - We need to have the parser or executor check for anomolies in the
 *   command arg such as a pipe at the end of the line etc */

#[derive(Debug, Copy, Clone)]
enum CmdAtom<'a> {
    Executable(&'a str),
    Arg(&'a str),
    Sym(&'a str),
}

#[derive(Debug)]
struct CommandParts<'d> {
    executable: CmdAtom<'d>,
    args: Vec<CmdAtom<'d>>,
}

impl<'d> CommandParts<'d> {
    fn new(exe: CmdAtom<'d>, arg_list: Vec<CmdAtom<'d>>) -> Self {
        CommandParts {
            executable: exe,
            args: arg_list,
        }
    }

    fn as_tuple(&self) -> (&CmdAtom<'d>, &Vec<CmdAtom<'d>>) {
        (&self.executable, &self.args)
    }
}

pub struct Parser<'p> {
    atoms: Vec<CmdAtom<'p>>,
}

impl<'p> Parser<'p> {
    /// Returns the string snippet if it is a symbol or None if not
    fn is_symbol(snippet: &'p str) -> Option<&'p str> {
        match snippet {
            // TODO: add more symbol types
            "|" => Some("|"),
            _ => None,
        }
    }

    fn parse_atoms_rec(
        mut string_parts: std::str::Split<'p, &str>,
    ) -> Vec<CmdAtom<'p>> {
        match string_parts.next() {
            Some(element) => {
                let mut so_far = Parser::parse_atoms_rec(string_parts);
                if so_far.is_empty() {
                    // This is one above the end
                    so_far.push(CmdAtom::Executable(element));
                } else {
                    // if we previously had a symbol we don't convert to arg
                    if let Some(CmdAtom::Executable(prev_element)) =
                        so_far.pop()
                    {
                        so_far.push(match Parser::is_symbol(prev_element) {
                            Some(a) => CmdAtom::Sym(a),
                            None => CmdAtom::Arg(prev_element),
                        });
                    }

                    // The last element should always be considered an executable
                    so_far.push(CmdAtom::Executable(element));
                }
                so_far
            }
            None => {
                // initialize the list
                Vec::new()
            }
        }
    }

    fn parse_atoms(
        string_parts: std::str::Split<'p, &str>,
    ) -> Vec<CmdAtom<'p>> {
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

    /// This is pivate as it's only ment to be called by the executor
    /* TODO: consider changing the return value so were not passing so much
     * data on the stack */
    fn get_parsed_commands(&self) -> Vec<CommandParts<'p>> {
        let mut ret = Vec::new();
        let mut curr_command: Option<CommandParts<'p>> = None;
        for (i, atom) in self.atoms.iter().enumerate() {
            match atom {
                CmdAtom::Executable(_) => {
                    match curr_command {
                        Some(mut cmd) => {
                            // update the one created in the Sym branch
                            cmd.executable = *atom;
                            curr_command = Some(cmd);
                        }
                        None => {
                            curr_command =
                                Some(CommandParts::new(*atom, Vec::new()));
                        }
                    }
                }
                CmdAtom::Arg(_) => {
                    if let Some(mut cmd) = curr_command {
                        cmd.args.push(*atom);
                        curr_command = Some(cmd);
                    } else {
                        panic!(
                            "There should be no args comming before a command"
                        );
                    }
                }
                CmdAtom::Sym(_) => {
                    if let Some(cmd) = curr_command {
                        ret.push(cmd);
                        curr_command = Some(CommandParts::new(
                            CmdAtom::Executable(""),
                            Vec::new(),
                        ));
                    } else {
                        panic!("There should be no Sym comming before curr_command has a value");
                    }
                }
            }
            if i == self.atoms.len() - 1 {
                if let Some(cmd) = curr_command {
                    // this is the last iteration
                    ret.push(cmd);
                    // to make the compiler happy
                    curr_command = None;
                }
            }
        }
        ret
    }

    /// This is pivate as it's only ment to be called by the executor
    fn get_parsed_syms(&self) -> Vec<CmdAtom<'p>> {
        self.atoms
            .iter()
            .filter(|atom| matches!(atom, CmdAtom::Sym(_)))
            .copied()
            .collect()
    }
}

pub struct CmdExecutor<'c> {
    parsed_line: &'c Parser<'c>,
}

impl<'c> CmdExecutor<'c> {
    pub fn new(to_exec: &'c Parser<'c>) -> Self {
        CmdExecutor {
            parsed_line: to_exec,
        }
    }

    /// Gets the command &str from the tuple form of command
    fn get_cmd_str(cmd_as_tuple: (&CmdAtom<'c>, &Vec<CmdAtom>)) -> &'c str {
        let executable_atom = cmd_as_tuple.0;
        if let CmdAtom::Executable(ret) = executable_atom {
            ret
        } else {
            panic!("There should not be any non Executable CmdAtom's here");
        }
    }

    /// Gets a vec of arg &str's from the tuple form of command
    fn get_cmd_args(
        cmd_as_tuple: (&CmdAtom, &Vec<CmdAtom<'c>>),
    ) -> Vec<&'c str> {
        let executable_atom_args = cmd_as_tuple.1;
        let mut ret = Vec::new();
        for executable_arg in executable_atom_args.iter() {
            if let CmdAtom::Arg(arg) = executable_arg {
                ret.push(*arg);
            } else {
                panic!("There should not be any Executable CmdAtom's here");
            }
        }
        ret
    }

    fn build_commands(command_parts: &[CommandParts<'c>]) -> Vec<Command> {
        command_parts
            .iter()
            .map(|cmd_part| -> Command {
                let cmd_part_tuple = cmd_part.as_tuple();
                let mut ret =
                    Command::new(CmdExecutor::get_cmd_str(cmd_part_tuple));
                ret.args(CmdExecutor::get_cmd_args(cmd_part_tuple));
                ret
            })
            .collect()
    }

    fn manage_command_startup(
        to_start: &mut Vec<Command>,
        sepparating_syms: &Vec<CmdAtom<'c>>,
    ) -> Result<(), std::io::Error> {
        if sepparating_syms.is_empty() && !to_start.is_empty() {
            let process = &mut to_start[0];
            match process.spawn() {
                Ok(mut child) => {
                    /* if it ran wait for it to finish and print
                     * any errors */
                    if let Err(e) = child.wait() {
                        Err(e)
                    } else {
                        Ok(())
                    }
                }
                Err(err) => Err(err),
            }
        } else if !sepparating_syms.is_empty() && !to_start.is_empty() {
            /* we should have either n-1 or n symbols where n is the number of
             * commands unless the last symbol is a | */
            // FIXME: put the command startup code here
            Ok(())
        } else {
            Ok(())
        }
    }

    //TODO: move the error messaging out into to main loop
    //fn run(&self) -> Result<(), std::io::Error> {
    pub fn run(&self) {
        let mut commands = CmdExecutor::build_commands(
            &self.parsed_line.get_parsed_commands(),
        );
        let symbols = self.parsed_line.get_parsed_syms();
        if !commands.is_empty() {
            if let Err(e) =
                CmdExecutor::manage_command_startup(&mut commands, &symbols)
            {
                println!("{}", e);
            }
        }
    }
}
