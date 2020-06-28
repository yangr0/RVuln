// Created by inc0gnit0, skript0r, p4yl0ad
// Latest update: 6/21/20
// Version: 0.0.4

// Dependencies
use chrono::Local; // 0.4.11
use isahc::prelude::*; // 0.9.3
use rayon::prelude::*; // 1.3.0
use regex::Regex; // 1.3.9
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::io::{BufRead, BufReader};
use std::time::Duration;

const BANNER: &str = "\x1b[95m    https://github.com/iinc0gnit0/RVuln

\x1b[94m██████╗ ██╗   ██╗██╗   ██╗██╗     ███╗   ██╗
██╔══██╗██║   ██║██║   ██║██║     ████╗  ██║
██████╔╝██║   ██║██║   ██║██║     ██╔██╗ ██║
██╔══██╗╚██╗ ██╔╝██║   ██║██║     ██║╚██╗██║
██║  ██║ ╚████╔╝ ╚██████╔╝███████╗██║ ╚████║
╚═╝  ╚═╝  ╚═══╝   ╚═════╝ ╚══════╝╚═╝  ╚═══╝
\x1b[91m════════════════════════════════════════════\x1b[0m\n";

const MENU: &str = "\x1b[92m[0]. Exit
[1]. XSS Scan";

// Main
fn main() -> std::io::Result<()> {
    // Check internet connection
    match connection() {
        Ok(send) => send,
        Err(_) => panic!("\x1b[91mConnection not found!\x1b[0m"),
    }
    // MENU
    println!("{}{}", BANNER, MENU);
    print!("\x1b[94m[ 𝓡𝓥𝓾𝓵𝓷 ] -> ");
    stdout().flush().unwrap();

    if input()? == "1" {
        match xss() {
            Ok(run) => run,
            Err(e) => println!("{}", e),
        }
    } else if input()? == "0" {
        println!("Exiting...");
        std::process::exit(1);
    } else {
        println!("Invalid Option")
    }

    Ok(())
}

// XSS
fn xss() -> std::io::Result<()> {
    print!("\n{}[2J", 27 as char);
    // Get Target
    print!("{}\x1b[94mTarget URL: ", BANNER);
    stdout().flush().unwrap();
    let target_url = &input()?;
    // Get Params
    print!("\x1b[91mQuery Parameters: ");
    stdout().flush().unwrap();
    let params = &input()?;
    // Get Wordlist
    print!("\x1b[93mPath to Wordlist: ");
    stdout().flush().unwrap();
    let wordlist = &input()?;
    // Verbosity
    print!("\x1b[95mVerbose ouput? [y/n]: ");
    stdout().flush().unwrap();
    let mut verbose = 0;
    if input()?.to_lowercase() == "y" {
        verbose = 1;
    } else if input()?.to_lowercase() == "n" {
        verbose = 0;
    }

    match read_files(target_url, wordlist, verbose, params) {
        Ok(run) => run,
        Err(e) => println!("{}", e),
    }

    Ok(())
}

// Input
fn input() -> std::io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = input.trim();
    Ok(input.to_owned())
}

// Read from wordlist and header file
fn read_files(target_host: &str, wordlist: &str, verbose: i8, params: &str) -> std::io::Result<()> {
    // Config variables
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
    let timeout = 15;
    // Reads payloads from file
    let mut payloads: Vec<String> = Vec::new();
    let fd = File::open(wordlist)?;
    for payload in BufReader::new(fd).lines() {
        let payload = payload.unwrap();
        let payload = payload.trim().to_owned();
        payloads.push(payload);
    }
    // Multi-thread request
    payloads.par_iter().for_each(|url_path| {
        match request(target_host, url_path, ua, verbose, timeout, params) {
            Ok(request) => request,
            Err(e) => println!("\x1b[91mSomething went wrong!\nError: {}", e),
        }
    });

    Ok(())
}

// Request
fn request(
    host: &str,
    payload: &str,
    ua: &str,
    verbose: i8,
    timeout: u64,
    params: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // This regex below replaces $ with the payload
    let re = Regex::new("\\$").unwrap();
    let url = format!("{}?{}", host, params);
    let url = re.replace(&url, payload);
    let url = url_encode(&url);
    // Sends request
    let request = Request::get(url.clone())
        .header("user-agent", ua)
        .timeout(Duration::new(timeout, 0))
        .body(())?
        .send()?
        .text()?;

    let source = request.contains(payload); // Check if source code contains payload

    let time = Local::now().format("%T");

    if verbose == 0 {
        if source == true {
            println!("\x1b[92m[{}] | [+] {}", time, url)
        } else if source == false {
            print!("");
        }
    } else if verbose == 1 {
        if source == true {
            println!("\x1b[92m[{}] | [+] {}", time, url)
        } else if source == false {
            println!("\x1b[91m[{}] | [-] {}", time, url)
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

// Check for internet connection
fn connection() -> Result<(), Box<dyn std::error::Error>> {
    Request::head("https://example.com/")
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
// https://portswigger-labs.net/xss/xss.php?x=$
