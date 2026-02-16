use std::io::{self, Write};
use reqwest::blocking::Client;
use html2text::from_read;

fn main() {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .build()
        .expect("Failed to create HTTP client");

    loop {
        print!("brodox > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let query = input.trim();

        if query == "exit" {
            break;
        }
        if query == "clear" {
            print!("{}[2J{}[1;1H", 27 as char, 27 as char);
            continue;
        }
        if query.is_empty() {
            continue;
        }

        let url = if query.contains('.') && !query.contains(' ') {
            if query.starts_with("http") {
                query.to_string()
            } else {
                format!("https://{}", query)
            }
        } else {
            let encoded_query = urlencoding::encode(query);
            format!("https://html.duckduckgo.com/html/?q={}", encoded_query)
        };

        match client.get(&url).send() {
            Ok(response) => {
                let text = from_read(response, 80);
                println!("\n{}\n", text);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
