use std::env;
use std::process::Command;

fn main() {
    // sorry for my crimes in advance, this is an example
    let mut args = env::args();
    args.next();

    match &*args.next().unwrap() {
        "binary-v7" => v7(),
        "binary-v8" => v8(),
        _ => println!("don't understand"),
    }
}

fn v7() {
    // build the main project
    let mut command = Command::new("cargo");
    command.arg("build")
        .current_dir("binary-v7");
    command.status().expect("failed to execute process");    

    // build the program
    let mut command = Command::new("cargo");
    command.arg("build")
        .arg("--target")
        .arg("thumbv7em-none-eabi")
        .current_dir("program");
    command.status().expect("failed to execute process");    

    // put it all together
    post_process();
}

fn v8() {
    // build the main project
    let mut command = Command::new("cargo");
    command.arg("build")
        .current_dir("binary-v8");
    command.status().expect("failed to execute process");    

    // build the program
    let mut command = Command::new("cargo");
    command.arg("build")
        .arg("--target")
        .arg("thumbv8m.main-none-eabihf")
        .current_dir("binary-v8");
    command.status().expect("failed to execute process");    

    // put it all together
    post_process();
}

fn post_process() {
    println!("some post-processing!");
}
