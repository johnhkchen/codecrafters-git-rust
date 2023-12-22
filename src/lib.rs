use std::fs;

pub fn run(args: &Vec<String>) -> String {
    if args.len() == 0 {
        return String::from("usage: git <command> [<args>]");
    }

    let command = args[0].as_str();
    match command {
        "init" => init(),
        "cat-file" => cat_file(args),
        _ => format!("git: '{command}' is not a git command. See 'git --help'."),
    }
}

#[cfg(not(tarpaulin_include))]
fn init() -> String {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    format!("Initialized git directory")
}

fn cat_file(args: &Vec<String>) -> String {
    print!("Args: ");
    for param in args {
        print!("{param} ");
    }
    println!("");
    format!("bar")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_args(line: &str) -> Vec<String> {
        // drop the first one because it's 'git'
        line.split_whitespace()
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn it_prints_usage() {
        let args = to_args("git");
        let results = run(&args);
        assert_eq!("usage: git <command> [<args>]", results);
    }

    #[test]
    fn it_rejects_invalid_commands() {
        // let args = vec!["barf".into()];
        let args = to_args("git barf");
        let results = run(&args);
        assert_eq!(
            "git: 'barf' is not a git command. See 'git --help'.",
            results
        );
    }

    #[ignore]
    #[test]
    fn it_runs_init() {
        // This test currently breaks because it does not clean up afterwards
        let args = to_args("git init");
        let results = run(&args);
        assert_eq!("Initialized git directory", results);
    }

    #[test]
    fn it_runs_cat_file() {
        // This test currently breaks because it does not clean up afterwards
        let args = to_args("git cat-file");
        let results = run(&args);
        assert_eq!("bar", results);
    }
}
