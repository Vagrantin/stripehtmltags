use std::env;
use std::fs;
use std::io::{self, Write};
use regex::Regex;

fn main() -> io::Result<()> {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <html_file>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    // Read the HTML file
    let html_content = fs::read_to_string(filename)?;
    
    // Create a regex to match HTML tags
    let tag_regex = Regex::new(r"<[^>]*>").unwrap();
    
    // Replace all HTML tags with newlines
    let content = tag_regex.replace_all(&html_content, "\n");
    
    // Process the content line by line
    let lines: Vec<&str> = content.split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    
    // Output the cleaned content to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for line in lines {
        writeln!(handle, "{}", line)?;
    }
    
    Ok(())
}
