mod utils;
use crate::utils::path;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    read_line();
}

fn read_line() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut input = String::new();
    let mut _result = handle_input(&mut input);
}

fn handle_input(input: &mut String) -> Result<(),Error> {
    let stdin = io::stdin();
    stdin.read_line(input).unwrap();
    trim(input);

    let input_list: Vec<&str>= input.split(" ").collect();
    
    match input_list[0] {
        "echo" => println!("{}", input_list[1..].join(" ")),
        "exit" => {
            if input_list.len() <= 1 {
                gracefull_exit(0)
            }
            let parsed_int = input_list[1].parse::<i32>();
            if parsed_int.is_err() {
                println!("invalid status code");
                gracefull_exit(1)
            }
            gracefull_exit(parsed_int.unwrap());
            return Ok(());
        },
        "type" => {
            if input_list.len() > 1 && 
                InputTypes::get_type(input_list[1]).is_some() {
                    println!("{} is a shell builtin", input_list[1]);
            } else if input_list.len() > 1 && 
                path::find_path(input_list[1]).is_some() {
                
                match path::find_path(input_list[1])
                    .unwrap()
                    .into_os_string()
                    .into_string() {
                        Ok(result) => println!("{}", result),
                        Err(..) =>
                            println!("{}: command not found", input_list[1]),
                }
            }
            else {
                println!("{}: not found", input_list[1]);
            }
        }
        "pwd" => {
            let path = env::current_dir()?;
            println!("{}", path.display());
        }
        "cd" => {
            let mut path = String::from("");
            if input_list.len() > 1 {
                path = input_list[1].to_string();

                if path.chars().next() == Some('~') && env::var("HOME").is_ok() {
                    path = path.replacen("~", &env::var("HOME").unwrap().to_string(),1);
                }
            }

            if Path::new(&path).exists() {
                let _ = env::set_current_dir(path);
            } else {
                println!("cd: {}: No such file or directory",path);
            }
        }
        _ => {
            let command = input_list[0];
            if command.is_empty() {
                println!("{}: command not found", input)
            }
            match path::find_path(command) {
                Some(_result) => {
                    let stdout = Command::new(input_list[0])
                        .args(input_list.into_iter().skip(1))
                        .stdout(Stdio::piped())
                        .spawn()?
                        .stdout
                        .ok_or_else(||
                            Error::new(
                                ErrorKind::Other,
                                "Could not capture standard output."))?;

                        let reader = BufReader::new(stdout);
                        reader
                            .lines()
                            .filter_map(|line| line.ok())
                            .for_each(|line| println!("{}", line));
                },
                None =>
                    println!("{}: command not found", input),
            }

        }
    }
    read_line();
    Ok(())
}

pub enum InputTypes {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd
}

impl InputTypes {
    fn get_type(s: &str) -> Option<Self> {
        match s {
           "echo" => Some(Self::Echo),
           "exit" => Some(Self::Exit),
           "type" => Some(Self::Type),
           "pwd" => Some(Self::Pwd),
           "cd" => Some(Self::Cd),
            _ => None,

        }
    }
}

fn gracefull_exit(code:i32) {
    process::exit(code);
}

fn trim(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

