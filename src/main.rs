
//  dev_startup
//  Command ran - intended to use as a shortcut tool to start the dev environment for custom development


use std::env;
use std::process::{Command, Stdio};
use std::io;
use std::path::PathBuf;


struct options {
    cwd: PathBuf,
    profile: String,
}

//  TODO
//  - Parse commands from args
//  - Build function for vscode opening

//  Opens vscode with specific options. Options are dictated from caller
fn open_vscode(options: options) -> io::Result<()> {
    //  Assumed `code` is in PATH - if not, this will fail
    let mut command = Command::new("code")
        .arg("--new-window")
        .arg("--profile")
        .arg(options.profile)
        .arg(options.cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;

    command.wait()?;
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
    let mut dev_options: options; 

    //  TODO Check Commands somehow
    //  parseCommands();

    //  iterate over commands w/ args
    for arg in args {
        match arg {
            cwd => continue,
            "rust" => {

            }
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
