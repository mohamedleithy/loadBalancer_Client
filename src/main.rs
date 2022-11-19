
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::fs;
use std::io::{Read, Write};
extern crate local_ip;
use std::time::{Duration, SystemTime};

fn main() -> std::io::Result<()>{
    {
        let mut f = File::options().append(true).open("reply.txt")?;
        let ip = local_ip::get().unwrap();
        let socket = UdpSocket::bind(ip.to_string() + ":2022")?;
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
       
        
      //  let mut f = File::open("test.text")?;

        // This is a mutable buffer that will be loaded from the socket
        // This is a short buffer so any long message will be cut off 
        let mut buf = [0; 60];
        

       // f.read(&mut buf)?;
    for i in 0..10 {
    
        
        let now = SystemTime::now();
        let t = format!("Sent::req{} time::{:?}\n", i, now);
        f.write_all( t.as_bytes())?;
        let t = format!("req{}", i);
        socket.send_to(t.as_bytes(), ip.to_string() +":2020")?;

    }

    for i in 0..10 {
        let (_, _) = socket.recv_from(&mut buf)?;
        let now = SystemTime::now();
        
        let mut msg = [0; 60];
        msg.copy_from_slice(&buf);
        msg.reverse();
        let msg = String::from_utf8((&msg).to_vec()).unwrap();
        let msg = msg.trim_matches(char::from(0));
        let t = format!("Recieved::{} time::{:?}\n", msg, now);
       f.write_all( t.as_bytes())?;
    }
        
        
    }

    Ok(())
    

}
