///  URL link sanitizer
///  Remove the source identifiers from the URL.
///  Example:
///  https://youtu.be/zfb1y8yn8QI?si=k5puuw5JPpjnzmMm
///  Remove everything after the ? symbol
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

// Known redirect parameter names used by various platforms
const REDIRECT_PARAMS: &[&str] = &["q", "url", "u", "dest", "destination", "redirect", "goto", "target"];

// Common tracking parameters to remove
const TRACKING_PARAMS: &[&str] = &[
    "si", "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content",
    "fbclid", "gclid", "msclkid", "mc_cid", "mc_eid",
    "ref", "source", "campaign", "referrer", "tracking",
    "event", "redir_token", "v" // YouTube specific
];

fn is_redirect_url(parsed_url: &Url) -> bool {
    let host = parsed_url.host_str().unwrap_or("");
    let path = parsed_url.path();
    
    // Check for known redirect patterns
    (host.contains("youtube.com") && path.contains("/redirect")) ||
    (host.contains("google.com") && path.contains("/url")) ||
    (host.contains("facebook.com") && path.contains("/l.php")) ||
    (host.contains("twitter.com") && path.contains("/t.co"))
}

fn extract_redirect_target(parsed_url: &Url) -> Option<String> {
    // Try to find the actual destination URL in query parameters
    for (key, value) in parsed_url.query_pairs() {
        if REDIRECT_PARAMS.contains(&key.as_ref()) {
            // The value might be a URL-encoded URL
            if value.starts_with("http://") || value.starts_with("https://") {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn remove_tracking_params(url: &str) -> Result<String, url::ParseError> {
    let mut parsed = Url::parse(url)?;
    
    // Collect parameters to keep (those not in the tracking list)
    let params_to_keep: Vec<(String, String)> = parsed
        .query_pairs()
        .filter(|(key, _)| !TRACKING_PARAMS.contains(&key.as_ref()))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    
    // Clear all query parameters
    parsed.query_pairs_mut().clear();
    
    // Add back only the non-tracking parameters
    for (key, value) in params_to_keep {
        parsed.query_pairs_mut().append_pair(&key, &value);
    }
    
    // Remove the '?' if no query parameters remain
    let result = parsed.to_string();
    if result.ends_with('?') {
        Ok(result[..result.len()-1].to_string())
    } else {
        Ok(result)
    }
}

fn sanitize_url(url: &str) -> Result<String, String> {
    // Try to parse the URL
    let parsed = Url::parse(url).map_err(|e| format!("Failed to parse URL: {}", e))?;
    
    // Check if this is a redirect URL
    if is_redirect_url(&parsed) {
        // Try to extract the actual destination
        if let Some(target) = extract_redirect_target(&parsed) {
            // Recursively sanitize the extracted URL
            return sanitize_url(&target);
        }
    }
    
    // Remove tracking parameters
    remove_tracking_params(url).map_err(|e| format!("Failed to process URL: {}", e))
}

fn main() {
    let args = Args::parse();
    
    match sanitize_url(&args.url) {
        Ok(sanitized_url) => println!("{}", sanitized_url),
        Err(e) => eprintln!("Error: {}", e),
    }
}

