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

fn init() -> String {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    format!("Initialized git directory")
}

fn cat_file(args: &Vec<String>) -> String {
    if args.len() == 1 {
        return format!("usage: git cat-file <object>");
    }
    let object_name = &args[2];
    format!("fatal: Not a valid object name {object_name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_query(line: &str) -> String {
        let args = line
            .split_whitespace()
            .skip(1)
            .map(str::to_string)
            .collect();

        run(&args)
    }

    #[test]
    fn it_prints_usage() {
        assert_eq!(run_query("git"), "usage: git <command> [<args>]",);
    }

    #[test]
    fn it_rejects_invalid_commands() {
        assert_eq!(
            run_query("git barf"),
            "git: 'barf' is not a git command. See 'git --help'."
        );
    }

    #[ignore]
    #[test]
    fn it_runs_init() {
        assert_eq!(run_query("git init"), "Initialized git directory");
    }

    #[test]
    fn it_runs_cat_file_usage() {
        assert_eq!(run_query("git cat-file"), "usage: git cat-file <object>");
    }

    #[test]
    fn it_runs_cat_file_emits_error_invalid_object_name() {
        assert_eq!(
            run_query("git cat-file -p 12345"),
            "fatal: Not a valid object name 12345"
        );
        assert_eq!(
            run_query("git cat-file -p 67890"),
            "fatal: Not a valid object name 67890"
        );
    }
}
