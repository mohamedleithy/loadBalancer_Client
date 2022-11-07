
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()>{
    {
        
        let socket = UdpSocket::bind("10.40.44.255:2022")?;
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
       
        
        let mut f = File::open("test.text")?;

        // This is a mutable buffer that will be loaded from the socket
        // This is a short buffer so any long message will be cut off 
        let mut buf = [0; 10];


        f.read(&mut buf)?;

        socket.send_to(b"req1", "10.40.44.255:2020")?;
        socket.send_to(b"req2", "10.40.44.255:2020")?;
        socket.send_to(b"req3", "10.40.44.255:2020")?;
        socket.send_to(b"req4", "10.40.44.255:2020")?;
        
        
        let (amt, src) = socket.recv_from(&mut buf)?;
        fs::write("reply1.txt",&buf)?;
        let (amt, src) = socket.recv_from(&mut buf)?;
        fs::write("reply2.txt",&buf)?;
        let (amt, src) = socket.recv_from(&mut buf)?;
        fs::write("reply3.txt",&buf)?;
        let (amt, src) = socket.recv_from(&mut buf)?;
        fs::write("reply4.txt",&buf)?;
    
    }

    Ok(())
    

}
