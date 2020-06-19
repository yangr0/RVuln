// Created by inc0gnit0, skript0r, p4yl0ad
// Latest update: 6/19/20
// Version: 0.0.3

// Dependencies
use isahc::prelude::*; // 0.9.3
use rayon::prelude::*;
use regex::Regex; // 1.3.9
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

// Main
fn main() -> std::io::Result<()> {
    // Check internet connection
    match connection() {
        Ok(send) => send,
        Err(_) => panic!("\x1b[91mConnection not found!\x1b[0m"),
    }
    let wordlist = "default.txt";
    let target_host = "https://portswigger-labs.net/xss/xss.php?x=$";
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
    let verbose = 1;
    let timeout = 15;
    // Reads payloads from file
    let mut payloads: Vec<String> = Vec::new();
    let fd = File::open(wordlist)?;
    for payload in BufReader::new(fd).lines() {
        let payload = payload.unwrap();
        let payload = payload.trim().to_owned();
        payloads.push(payload);
    }
    // Multi-threaded request
    payloads.par_iter().for_each(|url_path| {
        match request(target_host, url_path, ua, verbose, timeout) {
            Ok(request) => request,
            Err(e) => println!("\x1b[91mSomething went wrong!\nError: {}", e),
        }
    });

    Ok(())
}

// XSS
fn request(
    host: &str,
    payload: &str,
    ua: &str,
    verbose: i8,
    timeout: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // This regex below replaces $ with the payload
    let re = Regex::new("\\$").unwrap();
    let url = re.replace(host, payload);
    let url = url_encode(&url);
    // Sends request
    let request = Request::get(url.clone())
        .header("user-agent", ua)
        .timeout(Duration::new(timeout, 0))
        .body(())?
        .send()?
        .text()?;

    let source = request.contains(payload); // Check if source code contains payload
    if verbose == 0 {
        if source == true {
            println!("\x1b[92m[+] {}", url)
        } else if source == false {
            print!("")
        }
    } else if verbose == 1 {
        if source == true {
            println!("\x1b[92m[+] {}", url)
        } else if source == false {
            println!("\x1b[91m[-] {}", url)
        }
    }

    Ok(())
}
// Sanitizes URL
fn url_encode(data: &str) -> String {
    fn str_to_ascii_num(char: &str) -> u8 {
        let chars: Vec<_> = char.bytes().map(|c| c as char).collect();
        return chars[0] as u8;
    }
    let unsafe_chars: Vec<&str> = vec![
        " ", "'", "\"", ">", "<", "#", "%", "{", "}", "|", "\\", "^", "~", "[", "]", "+",
    ];
    let unsafe_nums: Vec<u8> = unsafe_chars.iter().map(|c| str_to_ascii_num(c)).collect();
    let supplied_nums: Vec<u8> = data.bytes().map(|b| b as u8).collect();
    let mut buffer = String::new();
    for num in supplied_nums {
        if unsafe_nums.contains(&num) {
            let sanitized = format!("%{:x?}", num).to_uppercase();
            buffer.push_str(&sanitized);
        } else {
            let sanitized = format!("{}", num as char);
            buffer.push_str(&sanitized);
        }
    }
    return buffer;
}
fn connection() -> Result<(), Box<dyn std::error::Error>> {
    Request::head("https://github.com")
        .timeout(Duration::new(15, 0))
        .body("")?
        .send()?;

    Ok(())
}
/*
red \x1b[91m
green \x1b[92m
yellow \x1b[93m
blue \x1b[94m
magenta \x1b[95m
reset \x1b[0m
*/
