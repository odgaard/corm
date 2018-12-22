extern crate clap;
//extern crate corm;

use clap::{Arg, App, AppSettings};
use std::process::Command;
use std::io;
use std::io::Write;
use std::process::Stdio;

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

    print!("Deleting {} file(s). ", files.len());
    std::io::stdout().flush().unwrap();
    println!("Total size: {}", df_sh(&files).trim());
    std::io::stdout().flush().unwrap();


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
            rm_sh(&files);
            println!("Deleted file succesfully");
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

fn df_sh(files: &Vec<&str>) -> String {
    let output = match Command::new("du")
                               .args(&["-s", "-c", "-h"])
                               .args(files)
                               .stdout(Stdio::piped())
                               .spawn() {
        Err(why) => panic!("Couldn't spawn df: {}", why),
        Ok(process) => process,
    };

    let output2 = 
    Command::new("grep")
            .arg("total$")
            .stdin(Stdio::piped())
            .output()
            .expect("Failed to execute");

    return String::from_utf8_lossy(&output2.stdout).replace("\ttotal", "");
}

fn rm_sh(files: &Vec<&str>) {
    Command::new("rm") 
            .args(files)
            .output()
            .expect("failed to execute process");
}
