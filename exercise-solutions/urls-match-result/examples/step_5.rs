fn parse_url(line: &str) -> Option<url::Url> {
    match url::Url::parse(&line) {
        Ok(u) => Some(u),
        Err(_e) => None,
    }
}

fn main() -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string("src/data/content.txt")?;
    println!("File opened and read");

    for line in file_contents.lines() {
        match parse_url(line) {
            Some(url) => {
                println!("Is a URL: {}", url);
            }
            None => {
                println!("Not a URL");
            }
        }
    }

    Ok(())
}

#[test]
fn correct_url() {
    assert!(parse_url("https://example.com").is_some())
}

#[test]
fn no_url() {
    assert!(parse_url("abcdf").is_none())
}
