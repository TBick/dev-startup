
//  dev_startup
//  Command ran - intended to use as a shortcut tool to start the dev environment for custom development


use std::env;
use std::process::{Command, Stdio};
use std::io;

fn main() {
    //  Entry Point


    //  What do I want to do on blank startup? What are the defaults
    //      Open VSCode in the current directory

    //  GOALS
    //      -Parse commands

    //  env variables
    let cwd = env::current_dir();
    let mut args = env::args().skip(1);

    //  TODO Check Commands somehow
    //  parseCommands();

    //  iterate over commands w/ args
    for arg in args {
        //  note: loop does not run if args empty

    }

    //  default behavior:
    //  Open up VSCode in the current directory
    //  Pick default profile ("Blank")
    if let Ok(current_dir) = cwd {
        //  Assumed `code` is in PATH - if not, this will fail
        println!("Opening VSCode in directory: {}", current_dir.display());
        let mut command = Command::new("code")
            .arg(current_dir)
            .arg("--profile")
            .arg("Blank")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to start VSCode");

            

    }


}
