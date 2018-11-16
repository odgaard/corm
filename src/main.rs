extern crate clap;
//extern crate corm;

use clap::{Arg, App, AppSettings};
use std::process::Command;
use std::io;
use std::io::Write;

//use corm::download::{ftp_download, http_download};
//use corm::utils;

// https://en.wikipedia.org/wiki/Corm

fn main() {
    let matches = App::new("Corm")
        .version("0.0.1")
        .author("Jacob Odgård Tørring <jacob.torring@gmail.com>")
        .about("Safe wrapper for rm written in Rust")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandsNegateReqs)

        // Corm
        .arg(Arg::with_name("input-file")
            .value_name("INPUT")
            .required(true)
            .min_values(1)
            .help("Set the input file to delete"))
        .arg(Arg::with_name("config-file")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("Sets a custom config file"))
            
        // Passthrough to original rm
        /*.subcommand(SubCommand::with_name("passthrough")
            .about("Passthrough to rm")
            .arg(Arg::with_name("passthrough-args")
                .value_name("PASSTHROUGH")
                .help("The arguments to be passed through to rm")
            ))
        */

        .get_matches();

    let files: Vec<_> = matches.values_of("input-file").unwrap().collect();

    let borrowed_string: &str = &files[0].to_owned();
    let mut owned_string2: String = "du -sch ".to_owned();
    let borrowed_string2: &str = " | grep total$";
    owned_string2.push_str(borrowed_string);
    owned_string2.push_str(borrowed_string2);
    print!("Deleting {} file(s). Total size: ", files.len());
    print!("{}", sh(&owned_string2));

    print!("Are you sure? [y/N] ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
        //    println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    match input.trim() {
        "y" | "Y" | "yes" | "Yes" | "YES" => {
            let mut owned_string: String = "rm ".to_owned();

            owned_string.push_str(borrowed_string);

            sh(&owned_string);
        },
        _ => println!("Canceled execution")
    }

    /*if let Some(ref matches) = matches.subcommand_matches("passthrough") {
        if matches.is_present("passthrough-args") {
            let files: Vec<_> = matches.values_of("passthrough-args").unwrap().collect();
            println!("Input {}", files[0]);
            sh(files[0]);
        }
    }*/
}

fn sh(command: &str) -> String {
    let output = 
        Command::new("sh") 
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process");

    // println!("Shell {}", command);
    return String::from_utf8_lossy(&output.stdout).replace("\ttotal", " ");
    // println!("Output {}", String::from_utf8_lossy(&output.stdout).replace("\ttotal", " "));
}
