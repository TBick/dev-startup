
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

    // TODO: Check Commands somehow

    let cwd = env::current_dir();
    println!("{:?}", cwd.unwrap());



}
