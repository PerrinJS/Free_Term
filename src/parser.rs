// TODO: impement actual argument parsing

pub struct Parser<'a> {
    command: &'a str,
    args: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(whole_command: &'a str) -> Self {
        let command_words = whole_command
            .trim()
            .split::<&str>(" ")
            .collect::<Vec<&str>>();
        // Convert them all to strings
        let mut arguments = command_words.clone();
        arguments.remove(0);
        Parser {
            command: command_words[0],
            args: arguments,
        }
    }

    pub fn get_command(&self) -> &'a str {
        self.command
    }

    pub fn get_args(&self) -> Vec<&'a str> {
        // I'm assuming this does what I think it does and just clones the refferences
        self.args.clone()
    }
}
