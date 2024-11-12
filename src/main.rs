use std::{env, net::{SocketAddr, TcpStream}, str::FromStr, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: pnock <ip> <port1> ... <portN>");
        return;
    }
    let ip = &args[1];
    
    for port in &args[2..] {
        let addr_str = format!("{}:{}", &ip, port);
        match SocketAddr::from_str(addr_str.as_str()) {
            Ok(addr) => {
                let _ = TcpStream::connect_timeout(&addr, Duration::from_millis(1));
            },
            Err(e) => {
                eprintln!("Could not connect to {}: {}", &addr_str, e);
                return;
            }
        }
    }
    println!("Done");
}
