mod lib;
mod constants;

use lib::LinesCodec;

use std::io;
use std::net::TcpStream;
use std::fs::OpenOptions;
use std::path::Path;

fn main() -> io::Result<()> {
    loop {
        if let Err(e) = handle_connect() {
            println!("Error in connection: {}\nReconnect? y/n", e);
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            match &response.trim()[..] {
                "Y" | "y" => continue,
                "N" | "n" => break,
                _ => {
                    println!("Invalid input, assuming n...");
                    break;
                }
            }
        }
        else{
            println!("Goodbye!");
            break;
        }
    }
    Ok(())
}

fn handle_connect() -> io::Result<()> {
    match TcpStream::connect("localhost:3333") {
        Ok(stream) => {
            println!("Successfully connected to server in port 3333");
            let mut codec = LinesCodec::new(stream)?;

            //Perform initial handshake
            let msg = constants::HELLO;
            codec.send_message(msg)?;
            println!("Performing initial handshake....");

            let data = codec.read_message()?;
            println!("{}", data);

            if &data == msg {
                //Initial Handshake successful
                println!("Initial handshake was successful !!");
                println!("Beginning user input loop... \n");

                // loop over user input
                loop {
                    println!("{}", constants::CURSOR);
                    let mut cmd = String::new();

                    // collect user input
                    io::stdin().read_line(&mut cmd)?;
                    cmd = cmd.trim().to_owned();
                    let cmd_vec : Vec<&str> = cmd.split(" ").collect();
                    match cmd_vec[0] {
                        // terminates the connection & the program
                        constants::QUIT => {
                            println!("Terminating connection to the server...");
                            codec.send_message(&cmd)?;
                            break;
                        },
                        // prints a one-line message
                        constants::DELETE | constants::MAKE_DIR | constants::REMOVE_DIR => {
                            codec.send_message(&cmd)?;
                            match codec.read_message()?.as_str() {
                                "Ok" => println!("{}", codec.read_message()?),
                                x => println!("{}", x),
                            }
                        },
                        // prints from a recieved list
                        constants::PRINT_DIR | constants::PRINT_HIDDEN | constants::SEARCH => {
                            codec.send_message(&cmd)?;
                            match codec.read_message()?.as_str() {
                                "Ok" => {
                                    let result_str = codec.read_message()?;
                                    // print each item on a new line
                                    println!("{}", constants::SERVER_RESPONSE);
                                    for e in result_str.split_whitespace() {
                                        println!("{}", e)
                                    }
                                },
                                x => println!("{}", x),
                            }
                        },
                        // prints help message
                        constants::HELP => print_help(),
                        // sends a local file to overwrite server file
                        constants::WRITE => {
                            if cmd_vec.len() > 2 {
                                let cmd = cmd_vec[0].to_owned() + " " + cmd_vec[1];
                                codec.send_message(&cmd)?;
                                if let Ok(mut file) = OpenOptions::new().read(true).write(true).create(false).open(Path::new(cmd_vec[2])){
                                    codec.send_file_as_str(&mut file)?;
                                    codec.set_timeout(5)?;
                                    match codec.read_message()?.as_str() {
                                        "Ok" => {
                                            match codec.read_file_as_str() {
                                                Ok(f) => println!("File Written:\n{}", f),
                                                Err(e) => println!("Error in reception of file: {}", e),
                                            }
                                        },
                                        x => println!("{}", x),
                                    }
                                    codec.set_timeout(0)?;
                                }
                                else{
                                    println!("Could not open specified file to send");
                                }
                            }
                            else{
                                println!("Error running command '{}': Missing file parameter", cmd_vec[0], );
                            }
                        },
                        // interacts w/ server file(s), returning final file on success
                        constants::READ | constants::COPY | constants::MOVE => {
                            codec.send_message(&cmd)?;
                            codec.set_timeout(5)?;
                            match codec.read_message()?.as_str() {
                                "Ok" => {
                                    match codec.read_file_as_str() {
                                        Ok(f) => println!("File Recieved:\n{}", f),
                                        Err(e) => println!("Error in reception of file: {}", e),
                                    }
                                },
                                x => println!("{}", x),
                            }
                            codec.set_timeout(0)?;
                        },
                        // invalid command detection moved to server side
                        _ => {
                            codec.send_message(&cmd)?;
                            println!("{}", codec.read_message()?);
                        },
                    }
                }
            }
            // initial handshake failed
            else {
                println!("Unexpected reply: {}", data);
            }
        }
        // connection failure message
        Err(e) => {
            println!("Failed to connect: {}", e);
            return Err(e);
        }
    }
    // connection terminated
    println!("Terminated.");
    Ok(())
}

// prints all file operations/commands available to user
fn print_help() {
    println!("*** File Operations / Commands ***");
    println!("[command]                       [output]");
    println!("read [file]                     returns the contents of a file");
    println!("write [file] [content]          writes the information in the contet file to the server file");
    println!("copy [from] [to]                copies a file to a new location");
    println!("move [from] [to]                moves a file to a new location");
    println!("del [file]                      deletes a file at a given location");
    println!("mkdir [directory]               creates a directory at the provided location");
    println!("rmdir [directory]               removes a provided directory and its contents");
    println!("printdir [directory]            prints all contents of specified directory");
    println!("printhidden                     prints all hidden files and directories of current working directory");
    println!("search [term]                   searches for file or directories containg the provided term");
    println!("help                            lists file operations / commands to user");
    println!("quit                            exits the server");
}