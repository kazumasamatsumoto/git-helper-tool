// src/main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new branch and switch to it
    QuickBranch {
        /// Name of the new branch
        branch_name: String,
    },
    /// Stage all changes, commit, and optionally push
    SmartCommit {
        /// Commit message
        #[arg(short, long)]
        message: String,
        /// Push changes after commit
        #[arg(short, long)]
        push: bool,
    },
    /// List recently active branches
    ListActiveBranches {
        /// Show branches modified within these many days
        #[arg(short, long, default_value_t = 7)]
        days: i32,
    },
}

fn run_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow::anyhow!(
            "Git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn quick_branch(branch_name: &str) -> Result<()> {
    run_git_command(&["checkout", "-b", branch_name])?;
    println!(
        "Successfully created and switched to branch: {}",
        branch_name
    );
    Ok(())
}

fn smart_commit(message: &str, push: bool) -> Result<()> {
    // Stage all changes
    run_git_command(&["add", "."])?;
    println!("Staged all changes");

    // Commit with message
    run_git_command(&["commit", "-m", message])?;
    println!("Successfully committed changes with message: {}", message);

    if push {
        run_git_command(&["push"])?;
        println!("Successfully pushed changes");
    }

    Ok(())
}

fn list_active_branches(days: i32) -> Result<()> {
    let output = run_git_command(&[
        "for-each-ref",
        "--sort=-committerdate",
        "--format=%(refname:short) (last modified: %(committerdate:relative))",
        "refs/heads/",
    ])?;

    println!("Recently active branches:");
    println!("{}", output);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::QuickBranch { branch_name } => quick_branch(branch_name),
        Commands::SmartCommit { message, push } => smart_commit(message, *push),
        Commands::ListActiveBranches { days } => list_active_branches(*days),
    }
}
