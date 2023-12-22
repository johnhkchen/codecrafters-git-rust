use git_starter_rust::run;
use std::env;

#[cfg(not(tarpaulin_include))]
fn echo_command(args: &Vec<String>) {
    print!("git ");
    for param in args {
        print!("{param} ");
    }
    println!("");
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    echo_command(&args);
    let message = run(&args);
    print!("{message}");
}
