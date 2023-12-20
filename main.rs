extern crate command;
use command::*;
use std::env;
use std::process::ExitCode;

fn usage(program: &str) {
    eprintln!("Usage: {program} <command>");
    eprintln!("Commands:");
    for command in COMMANDS.iter() {
        eprintln!(
            "     {name} - {description} ",
            name = command.name,
            description = command.description
        );
    }
}
#[command("hello", "Prints \"Ola meu mano!\" ")]
fn hello_command(_program: &str, mut _args: env::Args) -> ExitCode {
    println!("Ola meu mano!");
    return ExitCode::SUCCESS;
}
#[command("reverse", "Reverses characters of the arguments")]
fn reverse_command(_program: &str, mut args: env::Args) -> ExitCode {
    for arg in args {
        println!("{}", arg.chars().rev().collect::<String>());
    }
    return ExitCode::SUCCESS;
}

#[command("uppercase", "Brings all arguments to uppercase ")]
fn uppercase_command(_program: &str, mut args: env::Args) -> ExitCode {
    for arg in args {
        println!("{}", arg.to_uppercase());
    }
    return ExitCode::SUCCESS;
}

#[command("help", "Print this help message")]
fn help_command(program: &str, mut args: env::Args) -> ExitCode {
    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            println!(
                "{name} - {description}",
                name = command.name,
                description = command.description
            )
        } else {
            eprintln!("ERROR: command {command_name} is not found");
            return ExitCode::FAILURE;
        }
    } else {
        usage(&program);
    }
    return ExitCode::SUCCESS;
}

#[command("test", "Just a test command")]
fn test_command(_program: &str, mut args: env::Args)-> ExitCode{
    println!("urmom");
    return ExitCode::SUCCESS;
}

struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(&str, env::Args) -> ExitCode,
}
const COMMANDS: &[Command] = command_list!();

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args.next().expect("program");
    if let Some(command_name) = args.next() {
        let command = COMMANDS.iter().find(|command| command.name == command_name);

        match command {
            Some(command) => {
                (command.run)(&program, args);
            }
            _ => {
                usage(&program);
                eprintln!("ERROR: No command {command_name} is unknown");
                return ExitCode::FAILURE;
            }
        }
    } else {
        usage(&program);
        eprintln!("ERROR: No command provided");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
