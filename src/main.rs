
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()>{
    {
        
        let socket = UdpSocket::bind("172.20.10.5:34255")?;
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
       
        
        let mut f = File::open("test.text")?;

        // This is a mutable buffer that will be loaded from the socket
        // This is a short buffer so any long message will be cut off 
        let mut buf = [0; 10];


        f.read(&mut buf)?;

        socket.send_to(&buf, "172.20.10.5:5959")?;
        
        
        let (amt, src) = socket.recv_from(&mut buf)?;

        fs::write("reply.txt",&buf)?;
    
    }

    Ok(())
    

}
