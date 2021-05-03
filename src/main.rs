// use core::slice::SlicePattern;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
// use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;
mod util;
use util::*;
// use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Request {
    id: i8,
    payload: String,
}

#[derive(Serialize, Deserialize)]
struct Reply {
    id: i8,
    body: String,
}

fn send_reply(stream: &mut TcpStream, reply: Reply) -> TcpResult<()> {
    let serialized_req =
        bincode::serialize(&reply).map_err(|source| util::TcpError::SerializeError { source })?;
    send_bytes(stream, &serialized_req)?;
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> TcpResult<()> {
    //let mut data_len = [0u8; 4];
    let mut data = vec![0u8; 5000];
    //read_from_client(&mut stream, &mut data_len, 4)?;
    //let data_len = u32::from_ne_bytes(data_len) as usize;
    //println!("Receiving message of lenght:{}", data_len);
    let data_len = read_bytes(&mut stream, &mut data)?;
    let s = String::from_utf8_lossy(&data[0..data_len]);
    run_command(s);
    let reply = Reply {
        id: 1,
        body: String::from("pass"),
    };
    send_reply(&mut stream, reply)
}

fn run_command(s: Cow<str>) {
    println!("Running command: /usr/bin/stat {}", s);
    let output = Command::new("/usr/bin/stat")
        .arg(String::from(s))
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // we handle each connection in its own thread
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
