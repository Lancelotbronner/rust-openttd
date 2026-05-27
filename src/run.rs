use std::env;
use crate::cli::OriginalArgs;
use crate::fileio::determine_paths;

pub fn run(args: OriginalArgs) {
	let exe = env::current_exe().expect("failed to retrieve path to current executable");
	determine_paths(&exe, args.only_local_path)
}