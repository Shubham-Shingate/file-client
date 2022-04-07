mod lib;
mod constants;

use lib::LinesCodec;

use std::io::{self, BufReader, BufRead};
use std::net::TcpStream;
use std::process::exit;
use std::fs::File;
use std::path::Path;

use colored::*;

/*
    TODO Commands:
    SEARCH          - "search"  ---- searches files' content and filenames that match the given search input
 */

fn main() -> io::Result<()> {
    match TcpStream::connect("localhost:3333") {
        Ok(stream) => {
            println!("Successfully connected to server in port 3333");
            let mut codec = LinesCodec::new(stream)?;

            //Perform initial handshake
            let msg = constants::HELLO;
            codec.send_message(msg)?;
            println!("Performing initial handshake....");

            let mut data = String::new();
            data = codec.read_message()?;
            println!("{}", data);

            if &data == msg {
                //Initial Handshake successful
                println!("Initial handshake was successful !! \n Beginning user input loop... \n");

                // loop over user input
                loop {
                    println!("{}", constants::CURSOR);
                    let mut cmd = String::new();

                    // collect user input
                    io::stdin().read_line(&mut cmd).unwrap();
                    cmd = cmd.trim().to_owned();
                    let cmd_vec : Vec<&str> = cmd.split(" ").collect();
                    match cmd_vec[0] {
                        constants::QUIT => {
                            println!("Terminating connection to the server...");
                            codec.send_message(&cmd)?;
                            exit(0);
                        },
                        // prints all contents of given directory
                        // input: [printdir] [directory]
                        constants::PRINT_DIR => {
                            codec.send_message(&cmd)?;
                            let result_str = codec.read_message()?;
                            
                            println!("{}",constants::SERVER_RESPONSE);
                            for e in result_str.split_whitespace() {
                                println!("{}", e)
                            }
                        },
                        constants::PRINT_HIDDEN => {
                            // handle printing hidden files/dirs here
                            codec.send_message(&cmd)?;
                            let result_str = codec.read_message()?;
                            
                            println!("{}",constants::SERVER_RESPONSE); 
                            //println!("{}",result_str);  
                            for e in result_str.split_whitespace() {
                                println!("{}", e.bold().red());
                            }
                        },
                        constants::HELP => printHelp(),
                        constants::WRITE => {
                            let cmd = cmd_vec[0].to_owned() + " " + cmd_vec[1];
                            codec.send_message(&cmd);
                            if let Ok(file) = File::open(Path::new(cmd_vec[2])){
                                codec.send_file(&file);
                            }
                            else{
                                println!("Could not open specified file to send");
                            }
                            codec.set_timeout(5);
                            if let Ok(file) = codec.read_file(){
                                let file = BufReader::new(file);
                                println!("File Recieved:");
                                for i in file.lines(){
                                    println!("{}", i?);
                                }
                                codec.set_timeout(0);
                            }
                            else{
                                println!("No Response");
                            }
                        },
                        constants::READ | constants::COPY | constants::MOVE => {
                            codec.send_message(&cmd)?;
                            codec.set_timeout(5);
                            let file = BufReader::new(codec.read_file()?);
                            println!("File Recieved:");
                            for i in file.lines(){
                                println!("{}", i?);
                            }
                            codec.set_timeout(0);
                        },
                        _ => {
                            codec.send_message(&cmd)?;
                            println!("{}", codec.read_message()?);
                        },
                    }
                }
            } else {
                println!("Unexpected reply: {}", data);
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
    Ok(())
} // main

// prints all file operations/commands available to user
fn printHelp() {
    println!("*** File Operations / Commands ***");
    println!("[command]                       [output]");
    println!("printdir [directory]            prints all contents of specified directory");
    println!("printhidden                     prints all hidden files and directories of current working directory");
    println!("help                            lists file operations / commands to user");
    println!("quit                            exits the server");
}