extern crate xml;

use read_rss::xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;
use std::net::lookup_host;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

const SEC_RSS_URL: &'static str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom:80";

pub fn read_rss() {
    println!("Reading");
    let mut addrs_iter = SEC_RSS_URL
        .to_socket_addrs()
        .expect("Could not get IP addr");

    let addr: SocketAddr = addrs_iter.next().expect("asdf");
    let mut stream = TcpStream::connect(addr);
    match stream {
        Ok(mut s) => read_xml(&mut s),
        Err(_) => panic!("No Stream Opened"),
    };
}

pub fn read_xml(stream: &mut TcpStream) {
    let parser = EventReader::new(stream);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("name {}", name);
            }
            Ok(e) => println!("{:#?}", e),
            _ => println!("Nothing"),
        }
    }
}
