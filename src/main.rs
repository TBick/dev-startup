
//  dev_startup
//  Command ran - intended to use as a shortcut tool to start the dev environment for custom development


use std::{array, env};
use std::process::{Command, Stdio};
use std::io;
use std::path::PathBuf;
use std::io::Error;



struct dev_options {
    //  Options for dev startup
    cwd: PathBuf,
    vscode: vs_options,
}
struct vs_options {
    //  Options for vscode
    profile: String,
}

//  TODO
//  - Parse commands from args
//  - Build function for vscode opening

fn parseCommands(mut args: env::Args, cwd: Result<PathBuf, Error>) -> dev_options {
    //  Parse commands from args
    //  Return dev_options struct
    let mut options = dev_options {
        cwd: cwd.unwrap_or(env::current_dir().unwrap()),
        vscode: vs_options {
            profile: String::from("Blank"),
        },
    };
    
    for arg in args {
        match arg.as_str() {
            "--profile" => {
                if let Some(profile) = args.next() {}
            }
            &_ => return options
        }
    }

    return options;

}

//  Opens vscode with specific options. Options are dictated from caller
fn open_vscode(options: dev_options) -> io::Result<()> {
    //  Assumed `code` is in PATH - if not, this will fail
    let mut command = Command::new("code")
        .arg("--new-window")
        .arg("--profile")
        .arg(options.vscode.profile)
        .arg(options.cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn();

    Ok(())
}

fn main() {
    //  Entry Point


    //  What do I want to do on blank startup? What are the defaults
    //      Open VSCode in the current directory

    //  GOALS
    //      -Parse commands

    //  env variables
    let cwd= env::current_dir();
    let args = env::args();
    let mut vs_options: vs_options; 

    parseCommands(args, cwd);

    //  iterate over commands w/ args
    for arg in args {
        match arg {
            cwd => continue,
            
        }
        //  Print error if argument is present for now
        println!("Arguments are not allowed in this version. Found: {}", arg);
        return;

    }

    //  default behavior:
    //  Open up VSCode in the current directory
    //  Pick default profile ("Blank")
    if let Ok(current_dir) = cwd {
        //  Assumed `code` is in PATH - if not, this will fail
        println!("Opening VSCode in directory: {}", current_dir.display());
        let mut command = Command::new("code")
            .arg("--new-window")
            .arg("--profile")
            .arg("blank")
            .arg(current_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to start VSCode");



    }


}
