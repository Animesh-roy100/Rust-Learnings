use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // created instance of the Client struct from the reqwest crate to make HTTP request
    let client = Client::new();

    // Create a string query and set it to "ruts programming language" which will pass in the below-formatted string url
    let query = "rust+programming+language";
    let url = format!("https://www.google.com/search?q={}" , query);

    // Made a connection with the URL by sending an HTTP request
    let res = client.get(&url).send()?;

    // Try to get the HTML content returned from the HTTP request
    let html = res.text()?;

    // With the help of "Html" object from the "scraper" crate, we convert the extracted HTML into a document object model
    let fragment = Html::parse_document(&html);

    // With the help of "Selector" object from the "scraper" crate, we will select all the matching div with the class name g
    let selector = Selector::parse("div.g").unwrap();

    // Now iterate over all these selected divs to get the required search data.
    for element in fragment.select(&selector) {
        let title_selector = Selector::parse("h3").unwrap();
        let title_element = element.select(&title_selector).next().unwrap();
        let title = title_element.text().collect::<Vec<_>>().join("");

        let link_selector = Selector::parse(".yuRUbf > a").unwrap();
        let link_element = element.select(&link_selector).next().unwrap();
        let link = link_element.value().attr("href").unwrap();

        let snippet_selector = Selector::parse(".VwiC3b").unwrap();
        let snippet_element = element.select(&snippet_selector).next().unwrap();
        let snippet = snippet_element.text().collect::<Vec<_>>().join("");

        println!("Title: {}", title);
        println!("Link: {}", link);
        println!("Snippet: {}", snippet);
    }
    Ok(())
}
