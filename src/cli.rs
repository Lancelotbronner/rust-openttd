use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(flatten)]
    pub args: OriginalArgs,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {}

/// Original OpenTDD arguments
#[derive(Args, Debug)]
pub struct OriginalArgs {
    #[arg(short = 'I')]
    pub graphics_set: Option<String>,
    #[arg(short = 'S')]
    pub sounds_set: Option<String>,
    #[arg(short = 'M')]
    pub music_set: Option<String>,
    #[arg(short = 'm')]
    pub music_driver: Option<String>,
    #[arg(short = 's')]
    pub sound_driver: Option<String>,
    #[arg(short = 'v')]
    pub video_driver: Option<String>,
    #[arg(short = 'b')]
    pub blitter: Option<String>,
    #[arg(short = 'D')]
    pub dedicated: Option<String>,
    #[arg(short = 'f')]
    pub dedicated_forks: bool,
    #[arg(short = 'n')]
    pub connection: Option<String>,
    #[arg(short = 'p')]
    pub join_server_password: Option<String>,
    #[arg(short = 'r')]
    pub resolution: Option<String>,
    #[arg(short = 't')]
    pub start_year: Option<i32>,
    #[arg(short = 'd')]
    pub debug: Option<String>,
    #[arg(short = 'e')]
    pub editor: bool,
    #[arg(short = 'g')]
    pub file_to_saveload: Option<String>,
    #[arg(short = 'q')]
    pub validate_file: Option<String>,
    #[arg(short = 'Q')]
    pub skip_all_newgrf_scanning: bool,
    #[arg(short = 'G')]
    pub generation_seed: Option<u32>,
    #[arg(short = 'c')]
    pub config_file: Option<String>,
    #[arg(short = 'x')]
    pub disable_save_config: bool,
    #[arg(short = 'X')]
    pub only_local_path: bool,
}

#[test]
fn verify_cli() {
	use clap::CommandFactory;
	Cli::command().debug_assert();
}
