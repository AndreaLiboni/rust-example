use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34253")?;
        let mut buf = [0; 50];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = std::str::from_utf8(&buf[0..50]).expect("invalid utf-8 sequence");
        println!("{}", msg);
    }
    Ok(())

}