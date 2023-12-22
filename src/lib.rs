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
    if args.len() == 1 {
        return format!("usage: git cat-file <object>")
    }
    let object_name = &args[2];
    format!("fatal: Not a valid object name {object_name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_args(line: &str) -> Vec<String> {
        line.split_whitespace()
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    fn run_query(line: &str) -> String {
        run(&get_args(line))
    }

    #[test]
    fn it_prints_usage() {
        assert_eq!("usage: git <command> [<args>]", run_query("git"));
    }

    #[test]
    fn it_rejects_invalid_commands() {
        assert_eq!(
            "git: 'barf' is not a git command. See 'git --help'.",
            run_query("git barf")
        );
    }

    #[ignore]
    #[test]
    fn it_runs_init() {
        assert_eq!("Initialized git directory", run_query("git init"));
    }

    #[test]
    fn it_runs_cat_file_usage() {
        assert_eq!("usage: git cat-file <object>", run_query("git cat-file"));
    }

    #[test]
    fn it_runs_cat_file_emits_error_invalid_object_name() {
        assert_eq!("fatal: Not a valid object name 12345", run_query("git cat-file -p 12345"));
        assert_eq!("fatal: Not a valid object name 67890", run_query("git cat-file -p 67890"));
    }
}
