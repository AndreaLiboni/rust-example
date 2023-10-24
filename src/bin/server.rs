use std::env;
use std::net::UdpSocket;
use std::fs::OpenOptions;
use std::process;
mod lib;
use lib::{Contatto,byte_to_str};

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
        let args: Vec<String> = env::args().collect();
        let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("rubrica_data.json")?;
        let mut rubrica: Vec<Contatto> = serde_json::from_reader(&file)?;
        if args.len() == 2 && args[1] == "list"{
            println!("{:?}", rubrica);
            process::exit(0x0100);
        }
        let socket = UdpSocket::bind("127.0.0.1:34253")?;
        let mut buf = [0; 100];
        loop {
            let (_, sender) = socket.recv_from(&mut buf)?;
            let msg = byte_to_str(&buf);
            println!("{}", msg);
            let (cmd, data) = msg.split_once(':').unwrap();
            if cmd == "LIS"{
                let _ = socket.send_to(serde_json::to_string(&rubrica)?.as_bytes(), &sender);
            } else if cmd == "ADD"{
                let contatto_msg = serde_json::from_str(&data).unwrap();
                rubrica.push(contatto_msg);
                save_to_file(&rubrica);
            } else if cmd == "DEL"{
                let contatto_msg = serde_json::from_str(&data).unwrap();
                let pos = find_in_rubrica(&rubrica, &contatto_msg);
                if pos != -1{
                    rubrica.remove(pos as usize);
                    save_to_file(&rubrica);
                }
            }
            buf = [0; 100];
        }
    }
    Ok(())

}