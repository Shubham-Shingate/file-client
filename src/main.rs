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

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            //println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
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
                            // TODO - HANDLE ERROR WHEN USER INPUTS INCORRECT COMMAND !!!! 
                            // check which command collected from input 
                            if input.trim() == QUIT || input.trim() == "q" {
                                println!("exiting the server...");
                                exit(0);
                                //break
                            } else if input.trim() == PRINT_DIR || input.trim() == "pdir" {
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
                            } else if input.trim() == PRINT_HIDDEN || input.trim() == "ls -al" {
                                // walk current directory and print all hidden (.) directories and files
                                WalkDir::new(".")
                                    .into_iter()
                                    .filter_entry(|e| is_hidden(e))
                                    .filter_map(|v| v.ok())
                                    .for_each(|x| println!("{}", x.path().display()));
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