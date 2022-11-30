
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::io::{Read, Write};
extern crate local_ip;
use std::time::{Duration, SystemTime, Instant};
use std::env;
use std::io::prelude::*;
use std::thread;


fn main() -> std::io::Result<()>{
    {

      let mut children = vec![];
      
      for z in 500 .. 999 {

        let args: Vec<String> = env::args().collect();
        
        let filename = format!("./logs/clientsLog_2ServersUp_1ServerDown_500_client_10000req_100millis_1agent/client{}.txt", z); 
        let mut file = File::create(filename)?;

       children.push( thread::spawn( move || {
        let ip = local_ip::get().unwrap();
        let port = format!("{}{}",":9", z);
        // let socket = UdpSocket::bind(ip.to_string() + &port.to_string()).unwrap();
        // socket.set_write_timeout(Some(Duration::from_millis(50))).expect("set_write_timeout call failed");

        let msg22 = format!("failed to bind: {}" , port );
        let socket = UdpSocket::bind(ip.to_string() + &port.to_string()).expect(&msg22);

        let _ = socket.set_read_timeout(Some(Duration::from_millis(50))).expect(&msg22);

        let mut dropped_counter = 0;
        for n in 1..10000 {
          thread::sleep(Duration::from_millis(100));
        
          // Receives a single datagram message on the socket. If `buf` is too small to hold
          // the message, it will be cut off.
         
          
        //  let mut f = File::open("test.text")?;
  
          // This is a mutable buffer that will be loaded from the socket
          // This is a short buffer so any long message will be cut off 
          let mut buf = [0; 60];
          
  
         // f.read(&mut buf)?;
      
      
          let t = format!("req{}", z);
          let start = Instant::now();
          socket.send_to(t.as_bytes(), ip.to_string() +":2020").expect(&msg22);
  
      
          // let (_, _) = socket.recv_from(&mut buf).expect(&msg22);
          let manga = socket.recv_from(&mut buf);
          match manga {
            Ok((_,_src_addr)) => {
              let duration = start.elapsed();
  
              let timeElapsed = format!("time elapsed for request {} client {}: {:?} \n", n , z,duration);
              file.write_all(timeElapsed.as_bytes()).unwrap();
              
            }
            Err(_) => {
              dropped_counter = dropped_counter + 1; 

            }
          }
          
          
          let mut msg = [0; 60];
          msg.copy_from_slice(&buf);
          msg.reverse();
          let msg = String::from_utf8((&msg).to_vec()).unwrap();
          let msg = msg.trim_matches(char::from(0));
        }

        let dropped_msg = format!("{}", dropped_counter);
        file.write_all(dropped_msg.as_bytes()).unwrap();
        }
      ));
      
      }


      for child in children{

        let _ = child.join();
      }

     
    }

    Ok(())
    

}
