use std::{env};
use std::collections::HashSet;
use std::io::{stdout, Write, stdin};

use whoami::{username, devicename};
use colour::{cyan, green, yellow};
use regex;
use dirs::{home_dir};

mod builtins {
    pub mod basics;
}
use crate::builtins::basics::Basics;
use std::process::{Stdio, Child, Command};

const PIPE: char = '|';
const OUT_FILE: char = '>';
const IN_FILE: char = '<';
const OPS: [char; 3] = [PIPE, OUT_FILE, IN_FILE];

const BBIN: &str = "src/builtins/bin";


fn main() {
    let mut shell = Shell::new();

    let homedir: String  = home_dir().unwrap().to_str().unwrap().to_string();

    let c_split = regex::Regex::new(r"\||>|<").unwrap();

    loop {
        shell.print_startline_string();
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Unable to read input");

        let whole_command = input.trim();

        if whole_command.len() == 0 {
            continue;
        }

        let mut ops = shell.find_ops(whole_command.to_string());
        let mut commands = c_split.split(whole_command).into_iter().peekable();

        let mut previous_command: Option<Child> = None;

        while let Some(some_command) = commands.next() {
            let mut parts = some_command.trim().split_whitespace();
            let command = parts.next().expect("Failed to parse command");
            let args = parts;

            let stdin = previous_command.as_mut()
                .map_or(
                    Stdio::inherit(),
                    |output| Stdio::from(output.stdout.take().unwrap())
                );

            let stdout: Option<Stdio> = commands.peek()
                .map_or(
                    Some(Stdio::inherit()),
                    |_| match ops[0] {
                        PIPE => Some(Stdio::piped()),
                        _=> None,
                    }
                );

            if stdout.is_none() {
                eprintln!("Command operator {} is not supported", ops[0]);
                break;
            } else if ops.len() > 0 {
                ops.remove(0);
            }
            match command {
                "exit" => return,
                "cd" => {
                    let new_dir: &str = args.peekable().peek()
                        .map_or(
                            homedir.as_str(),
                            |x| {
                                if *x != "~" {
                                    return *x
                                }
                                return homedir.as_str();
                            }
                        );
                    shell.cd(new_dir);
                    previous_command = None;
                },
                "factor" => {
                    let command_result = Command::new(BBIN.to_owned() + "/qsmain")
                        .args(args)
                        .stdout(stdout.unwrap())
                        .spawn();

                    match command_result {
                        Ok(command_result) => {
                            previous_command = Some(command_result);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                },
                "icloudcli" => {
                    let command_result = Command::new(BBIN.to_owned() + "/icloudCLI")
                        .args(args)
                        .stdout(stdout.unwrap())
                        .stdin(stdin)
                        .spawn();

                    match command_result {
                        Ok(command_result) => {
                            previous_command = Some(command_result);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
                command => {
                    let command_result = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout.unwrap())
                        .spawn();

                    match command_result {
                        Ok(command_result) => {
                            previous_command = Some(command_result);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
            if let Some(ref mut child) = previous_command {
                match child.wait() {
                    Ok(_) => {},
                    Err(e) =>  {
                        eprintln!("Commmand {} failed with error code: {}", command, e);
                    }
                }
            }
        }

    }
}

struct Shell {
    usern: String,
    dname: String,
    pwd: String,
    operators: HashSet<char>,
}

impl Shell {
    pub fn new() -> Shell {
        return Shell {
            usern: username(),
            dname: devicename(),
            pwd: env::current_dir().unwrap().to_str().unwrap().to_string(),
            operators: OPS.iter().cloned().collect(),
        };
    }

    pub fn find_ops(&self, input: String) -> Vec<char> {
        let mut ops = Vec::new();
        for c in input.chars() {
            if self.operators.contains(&c) {
                ops.push(c);
            }
        }
        return ops;
    }

    pub fn print_startline_string(&self) {
        cyan!("{}", self.usern); print!("@"); green!("{}",self.dname);
        yellow!("{}", self.pwd); print!("$ ");
    }
}



#[cfg(test)]

mod shell_tests {
    use super::*;
    extern crate lazy_static;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref SHELL: Shell = Shell::new();
    }

    #[test]
    fn op_contain_test() {
        let s = "w|ord";
        let chs: Vec<char> = s.chars().collect();
        assert!(SHELL.operators.contains(&chs[1]));
    }

    #[test]
    fn find_ops_test() {
        let s = "ls | grep word > file.txt";
        let ops = SHELL.find_ops(s.to_string());
        assert_eq!(ops, vec!['|', '>']);
    }
}
