// Dependencies
use isahc;

fn main() {
    let url = isahc::get("https://google.com");
    println!("{:?}", url);
}
