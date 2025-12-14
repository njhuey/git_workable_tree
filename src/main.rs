use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "git workabletree",
    version,
    about = "
Git worktrees made workable
===========================

This CLI is designed to make using git worktrees more intuitive.
",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Clone a git repository and correctly setup the git worktree structure.
    Clone(CloneArgs),

    // Add a git worktree.
    Add(AddArgs),

    // Remove a git worktree.
    Remove(RmArgs),
}

#[derive(Parser, Debug)]
struct CloneArgs {
    repo: String,
}

#[derive(Parser, Debug)]
struct AddArgs {
    branch: String,

    #[arg(short, long)]
    new: bool,
}

#[derive(Parser, Debug)]
struct RmArgs {
    branch: String,

    #[arg(short, long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Clone(args) => {
            println!("Creating Repo")
        }
        Commands::Add(args) => {
            println!("Adding worktree")
        }
        Commands::Remove(args) => {
            println!("Removing worktree")
        }
    }
}
