mod lib;
mod constants;

use lib::LinesCodec;

// Commands the client can use
//const PRINT_DIR: &str = "printdir";        // lists contents of given directory
//const PRINT_HIDDEN: &str = "ls -al";       // lists all hidden (.) files and directories
//const QUIT: &str = "quit";                 // quits the file-client using exit()
//const HELP: &str = "help";                 // lists all possible file operations/commands
/*
    TODO Commands:
    SEARCH          - "search"  ---- searches files' content and filenames that match the given search input
 */
use std::io;
use std::net::TcpStream;
use std::process::exit;

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
                println!("Initial handshake was successful !! \n Beginning user input loop...");

                // loop over user input
                loop {
                    printHelp(); // prints all available file operations to user
                    println!("CMD>>");
                    let mut input = String::new();
                    // collect user input
                    io::stdin().read_line(&mut input).expect("Error reading input");

                    if input.trim() == constants::QUIT || input.trim() == "q" {
                        println!("exiting the server...");
                        exit(0);
                    } else if input.trim() == constants::PRINT_DIR || input.trim() == "pdir" {
                        // prompt for path to target directory
                        println!("Specify a directory to print the contents of:");
                        let mut dir_input = String::new();
                        io::stdin()
                            .read_line(&mut dir_input)
                            .expect("Error reading input");
                        let directory_name = format!("./{}", dir_input.trim());
                        println!("dir specified: {}", directory_name);

                        //stream.write((PRINT_DIR.to_owned() + "#" + &directory_name).as_bytes()).unwrap();
                    } else if input.trim() == constants::PRINT_HIDDEN || input.trim() == "ls -al" {
                        // prints hidden directories and files

                    } else if input.trim() == constants::HELP || input.trim() == "h" {
                        printHelp(); 
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
    println!("[command]           [shorthand]           [output]");
    println!("printdir            pdir                  prints all contents of a directory");
    println!("show-hidden         ls -al                prints all hidden files and directories of current working directory");
    println!("help                h                     lists file operations / commands to user");
    println!("quit                q                     exits the server");
}