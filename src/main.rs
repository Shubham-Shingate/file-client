use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::process::exit;
//use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;


// Commands the client can use
const PRINT_DIR: &str = "printdir";
const QUIT: &str = "quit";
/*
    TODO Commands:
    HELP            - "help"    ---- prints all possible commands the user can call
    PRINT_HIDDEN    - "ls -al"  ---- prints all hidden files and directories 
    SEARCH          - "search"  ---- searches files' content and filenames that match the given search input
 */

fn main() -> io::Result<()> {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                        println!("Beginning user input loop...");
                        // loop for receiving input by the user
                        loop {
                            println!("Please enter a command: ");
                            let mut input = String::new();
                            // collect user input
                            io::stdin().read_line(&mut input)
                                .expect("Error reading input");
                            /// TODO - HANDLE ERROR WHEN USER INPUTS INCORRECT COMMAND !!!! ///
                            // check which command collected from input 
                            if input.trim() == QUIT {
                                println!("exiting the server...");
                                exit(0);
                                //break
                            } else if input.trim() == PRINT_DIR {
                                // prompt for path to target directory
                                println!("Specify a directory to print the contents of:");
                                let mut dir_input = String::new();
                                io::stdin().read_line(&mut dir_input)
                                    .expect("Error reading input");
                                let directory_name = format!("./{}", dir_input.trim());  
                                println!("dir specified: {}", directory_name);

                                // print contents of given directory
                                // convert String(directory_name) to Path
                                let from_path = Path::new(&directory_name);    

                                // TODO - HANDLE DIRECTORY ERRORS (i.e., does not exist, etc) !!! ///
                                let mut entries = fs::read_dir(from_path)?
                                    .map(|res| res.map(|e| e.path()))
                                    .collect::<Result<Vec<_>, io::Error>>()?;

                                // The order in which `read_dir` returns entries is not guaranteed. If reproducible
                                // ordering is required the entries should be explicitly sorted.
                                entries.sort();
                                // The entries have now been sorted by their path.
                                for file in entries {
                                    println!("{:?}", file);
                                }
                            }
                        }

                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");

    Ok(())
}