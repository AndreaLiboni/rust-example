use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::fs::OpenOptions;

#[derive(Deserialize, Serialize, Debug)]
struct Contatto {
    name: String,
    surname: String,
    age: i32
}
impl PartialEq for Contatto {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.surname == other.surname && self.age == other.age
    }
}

fn byte_to_str(bytes: &[u8]) -> String{
    let mut new_str: String = String::from("");
    for c in bytes.iter() {
        if *c == 0x0 {break};
        new_str.push(*c as char);
    }
    new_str
}

fn save_to_file(rubrica: &Vec<Contatto>){
    let file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("rubrica_data.json").unwrap();
    let _ = serde_json::to_writer_pretty(&file, rubrica);
}

fn find_in_rubrica(rubrica: &Vec<Contatto>, contatto_to_find: &Contatto) -> i32{
    let mut i = 0;
    for cont in rubrica{
        if cont == contatto_to_find {return i;}
        i+=1;
    }
    return -1;
}

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34253")?;
        let mut buf = [0; 50];
        let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("rubrica_data.json")?;
        let mut rubrica: Vec<Contatto> = serde_json::from_reader(&file)?;
        loop {
            socket.recv_from(&mut buf)?;
            let msg = byte_to_str(&buf);
            let (cmd, data) = msg.split_once(':').unwrap();
            let contatto_msg = serde_json::from_str(&data).unwrap();
            if cmd == "ADD"{
                rubrica.push(contatto_msg);
                save_to_file(&rubrica);
            } else if cmd == "DEL"{
                let pos = find_in_rubrica(&rubrica, &contatto_msg);
                if pos != -1{
                    rubrica.remove(pos as usize);
                    save_to_file(&rubrica);
                }
            }
            //let msg_Contatto: Contatto = serde_json::from_str(&msg)?;
            //println!("Name: {}\nSurname: {}\nAge: {}", msg_Contatto.name, msg_Contatto.surname, msg_Contatto.age);
        }
    }
    Ok(())

}