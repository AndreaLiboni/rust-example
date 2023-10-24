use std::net::UdpSocket;
use std::io;
use std::env;
mod lib;
use lib::Contatto;
use lib::byte_to_str;


fn main() -> std::io::Result<()> {
    {
        let args: Vec<String> = env::args().collect();
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        let dest = "127.0.0.1:34253";
        let mut action = String::new();
        let mut contatto: Contatto = Contatto { name: String::new(), surname: String::new(), age: 0 };
        let mut age = String::new();
        let msg: String;
        if args.len() <= 1 {
            println!("Inserire il numero corrispondente all'azione da effettuare:\n 1) Aggiunta\n 2) Eliminazione\n 3) Lista");
            io::stdin().read_line(&mut action)?;
            match action.as_str(){
                "1\n" => action = String::from("ADD"),
                "2\n" => action = String::from("DEL"),
                "3\n" => action = String::from("LIS"),
                _ => println!("Error")
            };
            if action != "LIS" {
                println!("Inserisci il nome: ");
                io::stdin().read_line(&mut contatto.name)?;
                println!("Inserisci il cognome: ");
                io::stdin().read_line(&mut contatto.surname)?;
                println!("Inserisci l'età: ");
                io::stdin().read_line(&mut age)?;
                contatto.age = age.trim().parse().expect("Not a valid number");
                msg = format!(r#"{}:{{"name":"{}","surname":"{}","age":{}}}"#, action, contatto.name.strip_suffix("\n").unwrap(), contatto.surname.strip_suffix("\n").unwrap(), contatto.age);
            }else{
                msg = String::from("LIS:{}");
            }
        }else{
            if args[1] == "list" {
                msg = String::from("LIS:{}");
            }else {
                msg = format!(r#"{}:{{"name":"{}","surname":"{}","age":{}}}"#, args[1], args[2], args[3], args[4]);
            }
        }
        socket.send_to(msg.as_bytes(), &dest)?;
        if action == "LIS" || (args.len() > 1 && args[1] == "list"){
            let mut buf = [0; 200];
            socket.recv_from(&mut buf)?;
            println!("{}", byte_to_str(&buf));
        }

    }
    Ok(())

}