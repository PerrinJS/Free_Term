use std::env;

use crate::cmd_manager::{CommandParts, CmdAtom};

/* TO IMPLEMENT BUILTINS CREATE A BUILIN COMMAND OBJECT
 * HAVE SOME KIND OF LOOKUP TABLE OR WAY OF COMPARING THE STRING WE HAVE TO A
 * POTENTAL MATCH */

/* FIXME: this needs to be created as a class, as it needs to be initiallized.
 * we should make it a singleton, this is fine as it is readonly */

type BuiltinFn = fn(&[CmdAtom]) -> Box<String>;
const NUM_BUILTINS: usize = 1;
const BUILTINS: [(&str, BuiltinFn); NUM_BUILTINS] = [
    ("cd", builtin_cd),
];

pub fn is_builtin(cmd_str: &str) -> bool {
    for pair in BUILTINS.iter() {
        if pair.0 == cmd_str {
            return true;
        }
    }
    return false;
}

fn get_builtin(cmd_str: &str) -> Option<BuiltinFn> {
    assert!(is_builtin(cmd_str));
    let mut ret = None;
    for pair in BUILTINS.iter() {
        if pair.0 == cmd_str {
            ret = Some(pair.1)
        }
    }
    ret
}

pub fn handle_builtins(command: CommandParts) -> Option<Box<String>> {
    // This should not be used unless we have already checked that this is a builtin
    if let CmdAtom::Executable(e) = command.executable {
        assert!(is_builtin(e));
        if let Some(to_call) = get_builtin(e) {
            Some(to_call(&command.args))
        } else {
            None
        }
    } else {
        None
    }
}

// FIXME: this is just while we don't have an actuall cd directory handler
fn up_dir(curr_dir_chars: &mut Vec<char>) -> String {
    if curr_dir_chars.len() > 1 {
        // This is the end condidtion of up_dir_rec so we cant have it as the first char
        if *(curr_dir_chars.last().unwrap()) == '/' {
            curr_dir_chars.pop();
        }

        up_dir_rec(curr_dir_chars)
    } else if curr_dir_chars.len() == 1 {
        if *(curr_dir_chars.last().unwrap()) == '/' {
            curr_dir_chars.iter().collect::<String>()
        } else {
            // FIXME: this is just a random default value for now
            String::from('/')
        }
    } else {
        panic!("Up dir given bad dir string")
    }
}

// FIXME: this is just while we don't have an actuall cd directory handler
/// This is a recursive alg that takes a str and removes the top directory off
fn up_dir_rec(curr_dir_chars: &mut Vec<char>) -> String {
    if *(curr_dir_chars.last().unwrap()) == '/' {
        curr_dir_chars.iter().collect::<String>()
    } else {
        curr_dir_chars.pop();
        up_dir_rec(curr_dir_chars)
    }
}

fn builtin_cd(command: &[CmdAtom]) -> Box<String> {
    /* TODO: for cd we just need to check that the given operand is valid and
     * set the environment variable to that new position */
    let get_arg_str = |catom| {if let CmdAtom::Arg(e) = catom {e} else {""}};
    // FIXME: thease is just for testing 
    let is_up_dir = |dir_str| {dir_str == "../"};

    if command.len() == 1 {
        //There is no text output thus
        let mut ret = Box::new(String::new());
        let cmd_arg = get_arg_str(command[0]);

        // FIXME: this whole if statement is just for testing 
        if is_up_dir(cmd_arg) {
            match env::current_dir() {
                Ok(curr_dir) => {
                    let curr_dir_str = curr_dir.to_str().unwrap();
                    if curr_dir_str != "/" {
                        let mut curr_dir_ch = curr_dir_str
                            .chars()
                            .collect::<Vec<char>>();
                        let new_dir_string = up_dir(&mut curr_dir_ch);
                        if let Err(e) = env::set_current_dir(new_dir_string) {
                            *ret = e.to_string();
                        }
                    }
                },
                Err(e) => {
                    *ret = e.to_string();
                }
            }
        }

        ret

    } else {
        Box::new(String::from("too many arguments"))
    }
}


