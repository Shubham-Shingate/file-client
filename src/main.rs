mod lib;
mod constants;

use lib::LinesCodec;
mod file_ops;

use std::io;
use std::net::TcpStream;
use std::process::exit;
use colored::*;

fn main() -> io::Result<()> {
    match TcpStream::connect("localhost:3333") {
        Ok(stream) => {
            println!("Successfully connected to server in port 3333");
            let mut codec = LinesCodec::new(stream)?;

            file_ops::make_dir(constants::CLIENT_FILE_STORAGE)?;
            let current_dir = constants::CLIENT_FILE_STORAGE;

            //Perform initial handshake
            let msg = constants::HELLO;
            codec.send_message(msg)?;
            println!("Performing initial handshake....");

            let data = codec.read_message()?;
            println!("{}", data);

            if &data == msg {
                //Initial Handshake successful
                println!("Initial handshake was successful !! \n Beginning user input loop... \n");

                let mut logged_in = false;
                while !logged_in {
                    println!("Welcome to the File Client\nChoose your options\n1.Create a new account 2.Login into your account");
                    let mut choice=String::new();
                    io::stdin().read_line(&mut choice).unwrap();
                    choice = choice.trim().to_owned();
                    codec.send_message(&choice)?;
                    match choice.as_str() {

                        "1" => { //user creates a new account
                            println!("Please enter your username: ");
                            let mut username=String::new();
                            io::stdin().read_line(&mut username).unwrap();
                            username=username.trim().to_owned();
                            codec.send_message(&username)?;
                            println!("Please Enter your password: ");
                            let mut password=String::new();
                            io::stdin().read_line(&mut password).unwrap();
                            password=password.trim().to_owned();
                            codec.send_message(&password)?;
                            println!("Please Enter your email: ");
                            let mut email=String::new();
                            io::stdin().read_line(&mut email).unwrap();
                            email=email.trim().to_owned();
                            codec.send_message(&email)?;
                        },
                        "2" => { //User logs into existing account
                            println!("Enter your username: ");
                            let mut username=String::new();
                            io::stdin().read_line(&mut username).unwrap();
                            username=username.trim().to_owned();
                            codec.send_message(&username)?;
                            println!("Enter your password: ");
                            let mut password=String::new();
                            io::stdin().read_line(&mut password).unwrap();
                            password=password.trim().to_owned();
                            codec.send_message(&password)?;
                            let response = codec.read_message()?;
                            if response == "Success" {
                                println!("Server Response: Login Successfull");
                                logged_in = true;
                            } else {
                                println!("Server Response: {:?}", codec.read_message()?);
                            }
                        },
                        _ => {
                            println!("Invalid Choice");
                        }
                    }
                }

                // loop over user input
                loop {
                    println!("{}", constants::CURSOR);
                    let mut cmd = String::new();
                    // collect user input
                    io::stdin().read_line(&mut cmd).unwrap();
                    cmd = cmd.trim().to_owned();
                    let cmd_vec: Vec<&str> = cmd.split(" ").collect();

                    if cmd_vec[0] == constants::QUIT {
                        println!("Terminating connection to the server...");
                        codec.send_message(&cmd)?;
                        exit(0);
                    } else if cmd_vec[0] == constants::PRINT_DIR {
                        codec.send_message(&cmd)?;
                        let result_str = codec.read_message()?;

                        println!("{}",constants::SERVER_RESPONSE);
                        let result_vec: Vec<&str> = result_str.split("  ").collect();
                        result_vec.into_iter().for_each(|x| println!("{}", x));
                    } else if cmd_vec[0] == constants::PRINT_HIDDEN {
                        codec.send_message(&cmd)?;
                        let result_str = codec.read_message()?;

                        println!("{}",constants::SERVER_RESPONSE);
                        let result_vec: Vec<&str> = result_str.split(" ").collect();
                        result_vec.into_iter().for_each(|x| println!("{}", x.bold().red()));
                    } else if cmd_vec[0] == constants::MAKE_DIR {
                        codec.send_message(&cmd)?;
                        let result_str = codec.read_message()?;
                        println!("{}: {}", constants::SERVER_RESPONSE, result_str);
                    } else if cmd_vec[0] == constants::PUT_FILE {
                        codec.send_message(&cmd)?;
                        let file_data = file_ops::read_file(&(String::from(current_dir)+"/"+cmd_vec[1]))?;
                        codec.send_message(&file_data)?;
                        let result_str = codec.read_message()?;
                        println!("{}: {}", constants::SERVER_RESPONSE, result_str);
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
