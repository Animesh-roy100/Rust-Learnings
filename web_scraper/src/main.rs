fn main() {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    // Refined selector to more accurately target movie titles
    let title_selector = scraper::Selector::parse("h3").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    // println!("Found {} titles", titles.count());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
