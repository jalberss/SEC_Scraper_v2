extern crate xml;

use read_rss::xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;
use std::net::TcpStream;

const SEC_RSS_URL: &'static str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom";

pub fn read_rss() {
    println!("Reading");
    let stream = TcpStream::connect(SEC_RSS_URL);
    match stream {
        Ok(mut s) => read_xml(&mut s),
        Err(_) => panic!("No Stream Opened"),
    };
}

pub fn read_xml(stream: &mut TcpStream) {}
