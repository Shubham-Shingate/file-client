use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::process::exit;

// used for print directory file op
use std::io;
use std::fs::{self};
use std::path::Path;

// used for hidden dir file op
use walkdir::DirEntry as WalkDirEntry;
use walkdir::WalkDir;

// Commands the client can use
const PRINT_DIR: &str = "printdir";        // lists contents of given directory
const PRINT_HIDDEN: &str = "ls -al";       // lists all hidden (.) files and directories
const QUIT: &str = "quit";                 // quits the file-client using exit()
const HELP: &str = "help";                 // lists all possible file operations/commands
/*
    TODO Commands:
    SEARCH          - "search"  ---- searches files' content and filenames that match the given search input
 */

// returns true if file or directory is hidden; false otherwise
fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() -> io::Result<()> {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            //let mut msg = b"Hello!";

            //stream.write(msg).unwrap(); // send message to file-server
            //println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            //match stream.read_exact(&mut data) {
                //Ok(_) => {
                    //if &data == msg {
                        //println!("Reply is ok!");
                        println!("Beginning user input loop...");
                        // lists all possible file operations/commands
                        println!("*** File Operations / Commands ***");
                        println!("[command]           [shorthand]           [output]");
                        println!("printdir            pdir                  prints all contents of a directory");
                        println!("show-hidden         ls -al                prints all hidden files and directories of current working directory");
                        println!("help                h                     lists file operations / commands to user");
                        println!("quit                q                     exits the server");
                        // loop for receiving input by the user
                        loop {
                            println!("Please enter a command: ");
                            let mut input = String::new();
                            // collect user input
                            io::stdin().read_line(&mut input)
                                .expect("Error reading input");
                            stream.write(input.as_bytes()).unwrap();                                
                            // check which command collected from input 
                            if input.trim() == QUIT || input.trim() == "q" {
                                println!("exiting the server...");
                                exit(0);
                            } else if input.trim() == PRINT_DIR || input.trim() == "pdir" {
                                // prompt for path to target directory
                                println!("Specify a directory to print the contents of:");
                                let mut dir_input = String::new();
                                io::stdin().read_line(&mut dir_input)
                                    .expect("Error reading input");
                                let directory_name = format!("./{}", dir_input.trim());  
                                // println!("dir specified: {}", directory_name); 
                               
                                // convert String(directory_name) to Path
                                //let from_path = Path::new(&directory_name);    

                                // TODO send this path to file-server 
                                stream.write(directory_name.as_bytes()).unwrap();
                                let mut data = [0 as u8; 6]; // using 6 byte buffer                               
                                match stream.read_exact(&mut data) {
                                    Ok(_) => {
                                        if &data == directory_name.as_bytes() {
                                            println!("Reply is ok!");
                                            stream.write(directory_name.as_bytes()).unwrap();
                                        } else {
                                            let text = from_utf8(&data).unwrap();
                                            println!("Unexpected reply: {}", text);
                                        } // if
                                    }, // Ok
                                    Err(e) => {
                                        println!("Failed to receive data: {}", e);
                                    } // Err
                                } // match

                                // TODO receive Vec<String> from file-server and print here

                            } else if input.trim() == PRINT_HIDDEN || input.trim() == "ls -al" {
                                // prompt file-server to call handle_print_hidden()

                            } else if input.trim() == HELP || input.trim() == "h" {
                                // lists all possible file operations/commands
                                println!();
                                println!("*** File Operations / Commands ***");
                                println!("[command]           [shorthand]           [output]");
                                println!("printdir            pdir                  prints all contents of a directory");
                                println!("show-hidden         ls -al                prints all hidden files and directories of current working directory");
                                println!("help                h                     lists file operations / commands to user");
                                println!("quit                q                     exits the server");
                            } else {
                                println!("Please enter a valid file operation / command");
                            }
                        }
                    //} else {
                        // let text = from_utf8(&data).unwrap();
                        // println!("Unexpected reply: {}", text);
                    //} // if
                //}, // Ok
                //Err(e) => {
                    //println!("Failed to receive data: {}", e);
                //} // Err
            //} // match
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");

    Ok(())
}