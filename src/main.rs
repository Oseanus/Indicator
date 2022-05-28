use std::env;
use std::process;
use std::io;
use std::io::stdin;
use std::io::Read;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();

    match length {
        l if l > 1 => process::exit(64),
        l if l == 1 => match run_script(&args[0]) {
            Ok(e) => println!("{:?}", e),
            Err(e) => panic!("Problem opening the file: {:?}", e)
        }
        _ => run_prompt()
    }
}

/// Runs the interpreter with a given script.
/// # Arguments
/// - `script`: The script to run.
fn run_script(script: &String) -> io::Result<()> {
    let mut file = File::open(script)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    run(&source);

    Ok(())
}

/// Enters the command prompt and directly takes user input.
fn run_prompt() {
    let mut input = String::new();

    loop {
        print!("> ");
        stdin().read_line(&mut input).ok().expect("Failed to read input line.");

        if !input.is_empty() {
            run(&mut input);
        }
    }
}

/// Runs the given source.
/// # Arguments
/// - `input`: The source that should be run.
fn run(source: &String) {

}