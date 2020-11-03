use std::fs::{File, OpenOptions, create_dir};
use std::io::{BufReader, prelude::*, Error};
use std::path::Path;
use std::env;

mod connection;
use crate::connection::{Connection, TotalConnections};

mod input;
use crate::input::{Input};

fn create_file(input: &String) -> Result<File, Error> {
    File::create(input)
}

fn file_exists(input: &String) -> bool {
    Path::new(input).is_file()
}

fn open_file(input: &String) -> File {
    File::open(input).unwrap()
}

fn reader(input: &File) -> BufReader<&File> {
    BufReader::new(input)
}


fn main() {

    let input = Input::new(env::args().collect());

    let path = "./connection_blocker_data";

    match create_dir(path) {
        Err(e) => if e.to_string() != "File exists (os error 17)" { panic!(e) } 
        _ => ()
    }

    let formatted_wl = format!("{}{}", path, "/whitelisted_ips.txt");
    let formatted_bl = format!("{}{}", path, "/blocked_ips.txt");
    let formatted_al = &input.get_file();
    
    let whitelisted_ips = file_exists(&formatted_wl);
    let blocked_ips = file_exists(&formatted_bl);
    let auth_log = file_exists(&formatted_al);

    match (whitelisted_ips, blocked_ips, auth_log) {
        (true, true, true) => (),
        (_, _, false) => panic!("The file {} does not exist!", formatted_al),
        (false, false, _) => {
            match (create_file(&formatted_wl), create_file(&formatted_bl)) {
                (Ok(_), Ok(_)) => (),
                _ => panic!("Could not create file!")
            }
        }
        _ => ()
    }

    let whitelisted_ips = open_file(&formatted_wl);
    let blocked_ips = open_file(&formatted_bl);
    let auth_log = open_file(&formatted_al);

    let mut reader_wl = reader(&whitelisted_ips);
    let mut reader_bl = reader(&blocked_ips);
    let mut reader_al = reader(&auth_log);

    let mut tot_connections = TotalConnections::new();

    let mut whitelisted_ips = String::new();
    reader_wl.read_to_string(&mut whitelisted_ips).unwrap();

    for line in whitelisted_ips.lines() {
        let line = line.split(" ");    

        for word in line {
            if word == "" { continue; }
            tot_connections.push_wl(Connection::new(word.to_string()));
        }
    }

    let mut blocked_ips = String::new();
    reader_bl.read_to_string(&mut blocked_ips).unwrap();

    for line in blocked_ips.lines() {
        let line = line.split(" ");       

        for word in line {
            if word == "" { continue; }
            tot_connections.push(Connection::new(word.to_string()));
        }
    }

    let mut auth_log = String::new();
    reader_al.read_to_string(&mut auth_log).unwrap();

    for line in auth_log.lines() {
        let line = line.split(" ");

        for word in line {

            if check_ip_validity(word) {
                let connection = Connection::new(word.to_string());
                tot_connections.try_push(connection, &input);
            }
        }

    }

    let mut file = OpenOptions::new().write(true).open(formatted_bl).unwrap();

    let mut new_contents = String::new();
    for elem in tot_connections.get_bl() {
        new_contents = new_contents + " " + &elem.get_ip();
    }

    file.write_all(new_contents.as_bytes()).unwrap();

}

fn check_ip_validity(string: &str) -> bool {

    let split = string.split(".");
    let split_clone = split.clone();

    if split_clone.count() != 4 { return false }

    let mut iterator = split.into_iter();

    let w = iterator.next();
    let x = iterator.next();
    let y = iterator.next();
    let z = iterator.next();

    match (w, x, y, z) {
        (Some(w), Some(x), Some(y), Some(z)) => 
            is_string_numeric(w) & is_string_numeric(x) & is_string_numeric(y) & is_string_numeric(z),
        _ => false
    }

}

fn is_string_numeric(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_numeric() {
            return false
        }
    }
    true
}