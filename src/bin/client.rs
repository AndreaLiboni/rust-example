use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        let msg = "ciao";
        let dest = "127.0.0.1:34253";
        let _ = socket.send_to(msg.as_bytes(), &dest);
    }
    Ok(())

}