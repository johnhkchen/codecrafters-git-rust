use git_starter_rust::run;
use std::env;

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let message = run(&args);
    print!("{message}");
}
