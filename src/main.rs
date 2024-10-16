use anyhow::anyhow;
use shlex;

use clap::{Args, Parser, Subcommand};
use memchr::memmem;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run clean filter
    Clean(CleanArgs),
    /// Run smudge filter
    Smudge,

    /// Install doorman to repo git attributes
    Install(InstallArgs),
    /// Uninstall doorman from repo git attributes
    Uninstall(InstallArgs),

    /// Add doorman filter to global git config
    GlobalSetup,
    /// Remove doorman filter from global git config
    GlobalCleanup,
}

#[derive(Args)]
struct CleanArgs {
    /// Name of the file being updated ("%f")
    file: Option<String>,
}

#[derive(Args)]
struct InstallArgs {
    /// Git repository path to configure (defaults to git repo of current working directory, if any)
    #[arg(long, group = "target")]
    repo: Option<String>,

    /// Path to git attributes file to modify.  Use this to write filter configurations to
    /// .gitattributes instead of .git/info/attributes
    #[arg(long, group = "target")]
    attributes_file: Option<String>,

    /// Pattern to use to match files which are filtered
    #[arg(long, default_value = "*")]
    pattern: String,
}

impl InstallArgs {
    fn find_attributes_file(&self) -> anyhow::Result<PathBuf> {
        if let Some(attribute_file) = &self.attributes_file {
            Ok(std::path::PathBuf::from(attribute_file))
        } else {
            let repo_dir = std::path::PathBuf::from(match &self.repo {
                None => {
                    let toplevel = std::process::Command::new("git")
                        .args(["rev-parse", "--show-toplevel"])
                        .output()?;
                    if !toplevel.status.success() {
                        return Err(anyhow!("unable to locate top level of git repo.  is your current working directory in a git repo?"));
                    }
                    let mut out = String::from_utf8(toplevel.stdout)?;
                    out.truncate(out.trim_end().len());
                    out
                }
                Some(dir) => dir.clone(),
            });
            let git_info_dir = repo_dir.join(".git").join("info");
            if !git_info_dir.is_dir() {
                return Err(anyhow!(
                    "{} does not exist, is {} a git repo?",
                    git_info_dir.display(),
                    repo_dir.display()
                ));
            }
            Ok(git_info_dir.join("attributes"))
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clean(args) => {
            let file = args.file.as_deref().unwrap_or_else(|| "<stdin>");
            let mut buf = vec![];
            io::stdin().read_to_end(&mut buf)?;
            if memmem::find(&buf, b"XXX(tom)").is_some() {
                return Err(anyhow!("Forbidden pattern 'XXX(tom)' in file {}", file));
            }
            std::io::stdout().write_all(&buf)?;
        }
        Commands::Smudge => {
            std::io::copy(&mut std::io::stdin(), &mut std::io::stdout())?;
        }
        Commands::Install(args) => {
            let attributes_file = args.find_attributes_file()?;
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(attributes_file)?;
            writeln!(f, "{} filter=doorman", args.pattern)?;
        }
        Commands::Uninstall(args) => {
            let attributes_file = args.find_attributes_file()?;
            let attributes = std::fs::read_to_string(&attributes_file)?;
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(attributes_file)?;
            for line in attributes.lines() {
                if !line.contains("filter=doorman") {
                    write!(f, "{}", line)?;
                }
            }
        }
        Commands::GlobalSetup => {
            let whereami = std::env::current_exe()?;
            let quotedwhereami = shlex::try_quote(
                whereami
                    .to_str()
                    .ok_or_else(|| anyhow!("git doorman installed to non-unicode path"))?,
            )?;
            fn config_set(key: &str, val: &str) -> anyhow::Result<()> {
                Command::new("git")
                    .args(["config", "--global", key])
                    .arg(val)
                    .status()
                    .map_err(|e| e.into())
                    .and_then(|code| {
                        if code.success() {
                            Ok(())
                        } else {
                            Err(anyhow!("git config exited with error code {}", code))
                        }
                    })
            }
            config_set(
                "filter.doorman.clean",
                &format!("{} clean %f", quotedwhereami),
            )?;
            config_set(
                "filter.doorman.smudge",
                &format!("{} smudge", quotedwhereami),
            )?;
            config_set("filter.doorman.required", "true")?;
        }
        Commands::GlobalCleanup => {
            Command::new("git")
                .args(["config", "--global", "--remove-section", "filter.doorman"])
                .status()?;
        }
    }
    Ok(())
}
