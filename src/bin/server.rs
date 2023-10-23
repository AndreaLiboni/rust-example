use std::net::UdpSocket;
use std::str;
use serde_json::{Result, Value};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34253")?;
        let mut buf = [0; 50];
        /*let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/serde.json")?;*/
        loop {
            socket.recv_from(&mut buf)?;
            let mut msg: &str = str::from_utf8(&buf).unwrap();
            let mut actual_length = 0;
            for c in msg.chars() {
                if c == '\0' {break};
                actual_length+=1;
            }
            let mut str_msg = String::from(msg);
            str_msg.truncate(actual_length);
            msg = &str_msg[..];
            let json_data: Value = serde_json::from_str(&msg)?;
            println!("Name: {}\nSurname: {}\nAge: {}", json_data["name"], json_data["surname"], json_data["age"]);
            //let writer = json_data.to_writer_pretty();
            //serde_json::to_writer(&file, &10)?;
        }
    }
    Ok(())

}