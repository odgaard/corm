extern crate clap;
//extern crate corm;

use clap::{Arg, App, AppSettings, SubCommand};
// use std::process::Command;

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
        .subcommand(SubCommand::with_name("passthrough")
            .about("Passthrough to rm")
            .arg(Arg::with_name("passthrough-args")
                .value_name("PASSTHROUGH")
                .help("The arguments to be passed through to rm")
            ))
        .arg(Arg::with_name("input-file")
            .value_name("INPUT")
            .help("Set the input file to delete"))
        .arg(Arg::with_name("config-file")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("Sets a custom config file"))
        .get_matches();

//    let files: Vec<_> = matches.values_of("input-file").unwrap().collect();
    if let Some(ref matches) = matches.subcommand_matches("passthrough") {
        if matches.is_present("passthrough-args") {
            let files: Vec<_> = matches.values_of("passthrough-args").unwrap().collect();
            println!("{}", files[0]);
        }
    }
    //println!("Deleting this file: {}", files[0]);
/*

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {

        let mut owned_string: String = "rm ".to_owned();
        let another_string: String = files[0].to_owned();
        owned_string.push_str(&another_string);

        Command::new("sh")
                .arg("-c")
                .arg(owned_string)
                .output()
                .expect("failed to execute process")
   };
   println!("{}", String::from_utf8_lossy(&output.stdout));*/
}
