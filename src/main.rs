
//  dev_startup
//  Command ran - intended to use as a shortcut tool to start the dev environment for custom development


use std::env;
use std::process::{Command, Stdio};
use std::io;
use std::path::PathBuf;
use std::io::Error;

//  Structs for options
//  dev_options - options for dev startup
//  vs_options - options for vscode
/////////////////////////////////////////////////////////////////
struct DevOptions {
    //  Options for dev startup
    cwd: PathBuf,
    vscode: VsOptions,
}
struct VsOptions {
    //  Options for vscode
    profile: String,
    git: bool,
    //extensions: Vec<String>
}
/////////////////////////////////////////////////////////////////





//  Parse commands from args
//  Returns dev_options struct
//  If no args, return default options
/////////////////////////////////////////////////////////////////
fn parse_commands(mut args: env::Args, cwd: Result<PathBuf, Error>) -> DevOptions {
    //  Parse commands from args
    //  Return dev_options struct
    let mut options = DevOptions {
        cwd: cwd.unwrap_or(env::current_dir().unwrap()),
        vscode: VsOptions {
            profile: String::from("Blank"),
            git: false,
            //extensions: vec![]
        },
    };
    
    while let Some(arg) = args.next() {
        if arg == "dev_startup" {
            continue; // Skip the first argument which is the program name
        }
        match arg.as_str() {
            "--profile" | "-p" => {
                if let Some(profile_name) = args.next() {
                    match profile_name.as_str() {
                        "blank" | "Blank" => options.vscode.profile = String::from("Blank"),
                        "rust" | "Rust" => options.vscode.profile = String::from("Rust"),
                        _ => {
                            println!("Unknown profile: {}. Using default 'Blank' profile.", profile_name);
                            options.vscode.profile = String::from("Blank");
                        }
                    }
                }
            }
            "--cwd" | "-c" => {
                if let Some(path_str) = args.next() {
                    let path = PathBuf::from(path_str);
                    if path.exists() && path.is_dir() {
                        options.cwd = path;
                    } else {
                        println!("Invalid directory specified. Using current directory instead.");
                    }
                }
            }
            "--git" | "-g" => {
                options.vscode.git = true;
            }
            "--vs-extensions" | "-e" => {
                if let Some(ext_list) = args.next() {
                    println!("This feature is not implemented yet {}. Terminating...", ext_list);
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                println!("dev_startup - A tool to setup dev environment with specific VSCode settings and other necessary resources.");
                println!("Usage:");
                println!("  --profile, -p [profile_name]   Specify the VSCode profile to use (e.g., Blank, Rust)");
                println!("  --cwd, -c [path]               Specify the working directory (defaults to current directory)");
                println!("  --git, -g                      Enable git integration");
                println!("  --vs-extensions, -e [ext1,ext2] Specify a comma-separated list of VSCode extensions to install (not implemented yet)");
                println!("  --help, -h                     Show this help message");
                println!("  --cwd, -c [path]               Specify the working directory (defaults to current directory)");
                std::process::exit(0);
            }
            _ => {
                println!("Unknown argument: {}. Ignoring.", arg);
            }
        }
    }

    return options;

}
/////////////////////////////////////////////////////////////////




//  Helper functions for git, vscode, etc
/////////////////////////////////////////////////////////////////

//  Opens vscode with specific options. Options are dictated from caller
fn open_vscode(options: &DevOptions) -> io::Result<()> {
    //  Assumed `code` is in PATH - if not, this will fail
    let window =Command::new("code")
        .arg("--new-window")
        .arg("--profile")
        .arg(&options.vscode.profile)
        .arg(&options.cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn();
    
    match window {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to open VSCode: {}", e);
            Err(e)
        }
    }
}

//  Initializes git if requested
//  If git already exists, verify status and pull latest changes
//  If git does not exist, initialize new repo
fn git_initialize(options: &DevOptions) -> io::Result<()> {
    //  Check if .git exists in cwd
    let git_path = options.cwd.join(".git");
    if git_path.exists() && git_path.is_dir() {
        //  Git repo exists - verify status and pull latest changes
        let status = Command::new("git")
            .arg("-C") 
            .arg(&options.cwd)
            .arg("status")
            .output()?;
        if !status.status.success() {
            eprintln!("Failed to get git status.");
            return Err(io::Error::new(io::ErrorKind::Other, "Git status failed"));
        }
        println!("Git repository found. Pulling latest changes...");
        let pull = Command::new("git")
            .arg("-C")
            .arg(&options.cwd)
            .arg("pull")
            .output()?;
        if !pull.status.success() {
            eprintln!("Failed to pull latest changes.");
            return Err(io::Error::new(io::ErrorKind::Other, "Git pull failed"));
        }
        println!("Successfully pulled latest changes.");
    } else {
        //  Git repo does not exist - initialize new repo
        println!("No git repository found. Initializing new git repository...");
        let init = Command::new("git")
            .arg("-C")
            .arg(&options.cwd)
            .arg("init")
            .output()?;
        if !init.status.success() {
            eprintln!("Failed to initialize git repository.");
            return Err(io::Error::new(io::ErrorKind::Other, "Git init failed"));
        }
        println!("Successfully initialized new git repository.");
        println!("You may want to add a remote repository and make your first commit.");
        println!("Use 'git remote add origin <url>' to add a remote.");
        println!("To create remote repository, run 'gh repo create <repo-name>' if you have GitHub CLI installed.\n\n");
    }
    Ok(())
}
/////////////////////////////////////////////////////////////////





////////////////////////////////////////////////////////////////
fn main() {
    //  Entry Point

    //  env variables
    let cwd= env::current_dir();
    let args = env::args();
    let dev_options = parse_commands(args, cwd);

    //  Open VSCode with options
    match open_vscode(&dev_options) {
        Ok(_) => println!("VSCode opened successfully."),
        Err(e) => {
            eprintln!("Failed to open VSCode: {}", e);
            std::process::exit(0);
        }
    }

    //  Safe to assume vscode window opened sucessfully

    //  Handle git operations if requested
    if dev_options.vscode.git {
        // Check if git is installed
        if let Err(_) = Command::new("git").arg("--version").output() {
            eprintln!("Git is not installed or not found in PATH. Skipping git initialization.");
        } else {
            match git_initialize(&dev_options) {
                Ok(_) => println!("Git operations completed successfully."),
                Err(e) => eprintln!("Git operations failed: {}", e),
            }
        }
    }

    // TODO: Add browser opening for documentation if needed.
    //  Ideas for browser opening:
    //  - Open Rust docs if profile is Rust
    //  - Open project documentation if exists in cwd
    //  - Open any other relevant resources (Rust Book, etc)


}
