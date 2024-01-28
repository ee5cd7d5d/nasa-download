use error_chain::error_chain;
use html_parser::Dom;

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
    println!("Body:\n{}", body);
    let json_dom = Dom::parse(body_no_header.as_str());
    let maybe_pretty_json = match json_dom {
        Ok(json) => json.to_json_pretty(),
        Err(error) => panic!("{}", error), // TODO: handle header removal on error here
    };
    let pretty_json = match maybe_pretty_json {
        Ok(json) => json,
        Err(error) => panic!("{}", error),
    };
    println!("{}", pretty_json);
    Ok(())
}
