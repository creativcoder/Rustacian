use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

fn get_tld_server(tld:&str) -> Option<String> {
    let mut stream = TcpStream::connect("whois.iana.org:43").unwrap();
    stream.write_all(format!("{}\n",tld).as_bytes()).unwrap(); // sending tld
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}",line);
        let parts:Vec<String> = line.splitn(2,":").map(|x| x.to_string()).collect(); 
        if parts.len() == 2 {
            if parts[0].to_lowercase() == "whois" {
                return Some(parts[1].trim().to_string());   
            }
        }
    }
    return None;
}

fn main() {
    match get_tld_server("rust-lang.org") {
        Some(server) => {
            println!("Origin server: {}",server);
        },
        None => {
            println!("Server resolution failed");
        }
    }
}
