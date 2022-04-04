mod lib;
mod constants;

use lib::LinesCodec;

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
                    println!("CMD>>");
                    let mut input = String::new();
                    // collect user input
                    io::stdin().read_line(&mut input).expect("Error reading input");

                    if input.trim() == constants::QUIT {
                        println!("exiting the server...");
                        exit(0);
                    } else if input.trim() == constants::PRINT_DIR {
                        // prompt for path to target directory
                        println!("Specify a directory to print the contents of:");
                        let mut dir_input = String::new();
                        io::stdin()
                            .read_line(&mut dir_input)
                            .expect("Error reading input");
                        let directory_name = format!("./{}", dir_input.trim());
                        println!("dir specified: {}", directory_name);

                        //stream.write((PRINT_DIR.to_owned() + "#" + &directory_name).as_bytes()).unwrap();
                    } else {

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
}
