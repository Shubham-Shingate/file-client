
use std::fs;
use std::io;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

pub fn make_dir(dir_name: &str) -> io::Result<()> {
    fs::create_dir_all(dir_name)?;
    println!("NewDir directory created successfully\n");
    Ok(())
}

pub fn read_file(file_name: &str) -> io::Result<String> {
    let mut file_ref = File::open(file_name).unwrap();
    let mut data = String::new();
    file_ref.read_to_string(&mut data).unwrap();
    Ok(data)
}

pub fn write_file(file_name: &str, file_data: &str) -> io::Result<()> {
    let mut file_ref = OpenOptions::new().write(true).create(true).open(file_name).expect("Unable to open file");
    file_ref.write_all(file_data.as_bytes())?;
    Ok(())
}