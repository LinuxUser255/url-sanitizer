///  URL link sanitizer
///  Remove the source identifiers from the URL.
///  Example:
///  https://youtu.be/zfb1y8yn8QI?si=k5puuw5JPpjnzmMm
///  Remove everything after the ? symbol

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    url: String,
}

fn sanitize_url(url: &str) -> String {
    if let Some(q_pos) = url.find('?') {
        url[..q_pos].to_string()
    } else {
        url.to_string()
    }
}

fn main() {
    let args = Args::parse();
    let sanitized_url = sanitize_url(&args.url);
    println!("Sanitized URL: {}", sanitized_url);
}

