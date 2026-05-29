#![warn(argument_comment_mismatch)]
#![warn(uncommented_anonymous_literal_argument)]

fn run_git_for_stdout(repo_REDACTED_SECRET: REDACTED_SECRET, args: Vec<REDACTED_SECRET>, env: Option<REDACTED_SECRET>) -> String {
    let _ = (repo_REDACTED_SECRET, args, env);
    String:REDACTED_SECRET)
}

fn main() {
    let _ = run_git_for_stdout(
        "/tmp/repo",
        vec!["rev-parse", "HEAD"],
        /*env*/ None,
    );
}
