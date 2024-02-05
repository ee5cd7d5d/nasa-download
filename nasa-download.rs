use error_chain::error_chain;
use scraper::{Html, Selector};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://planetarydata.jpl.nasa.gov/img/data/juno/JNOJNC_0001/").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let body_lines: Vec<&str> = body.split('\n').collect();
    let body_no_header = body_lines[2..].join("\n");
    let html = Html::parse_fragment(&body_no_header);
    let table_selector = Selector::parse(r#"table[id="indexlist"]"#).unwrap(); // Here unwrapping is ok
    let table = html.select(&table_selector).next().unwrap(); // Here it is not, panic
    println!("{}", table.inner_html());
    let row_selector = Selector::parse("tr").unwrap();
    let format_selector = Selector::parse("img").unwrap();
    let href_selector = Selector::parse("a").unwrap();
    for row in table.select(&row_selector) {
        println!("{}", row.html());
        let href = row.select(&href_selector).next().unwrap();
        println!(
            "\n{} --- {} \n",
            href.html(),
            href.value().attr("href").unwrap()
        );
        let format = href.select(&format_selector).next();
        match format {
            Some(format) => println!("\n FORM {}", format.value().attr("alt").unwrap()),
            None => println!("\n NOT FOUND"),
        }
    }
    Ok(())
}
