use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value_t = 0)]
    depth: i32,

    #[arg(short, long)]
    extensions: Option<Vec<String>>,
}

struct _NasaPage {
    url: String,
    content: Option<String>,
    depth: u32,
    dirs: Option<Vec<Self>>,
    files: Option<Vec<String>>,
}

static TABLE_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse(r#"table[id="indexlist"]"#).unwrap());
fn find_table_in_content(html: &Html) -> Option<scraper::ElementRef> {
    let table = html.select(&TABLE_SELECTOR).next()?;
    Some(table)
}

static ROW_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("tr").unwrap());
static FORMAT_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("img").unwrap());
static HREF_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("a").unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    //let cli = Cli::parse();
    let res = reqwest::get("https://planetarydata.jpl.nasa.gov/img/data/juno/JNOJNC_0001/").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let html = Html::parse_fragment(&body);
    let table = find_table_in_content(&html).unwrap();
    println!("{}", table.inner_html());
    for row in table.select(&ROW_SELECTOR) {
        println!("{}", row.html());
        let href = row.select(&HREF_SELECTOR).next().unwrap();
        println!(
            "\n{} --- {} \n",
            href.html(),
            href.value().attr("href").unwrap()
        );
        let format = href.select(&FORMAT_SELECTOR).next();
        match format {
            Some(format) => println!("\n FORM {}", format.value().attr("alt").unwrap()),
            None => println!("\n NOT FOUND"),
        }
    }
    Ok(())
}
