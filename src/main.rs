use clap::{App, Arg};
use easy_scraper::Pattern;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ac-editorial")
        .arg(
            Arg::new("contest_id")
                .value_name("contest_id")
                .required(true),
        )
        .get_matches();
    let contest_id = matches.value_of("contest_id").unwrap();
    let contest_url = format!("https://atcoder.jp/contests/{}", contest_id);
    let client = reqwest::ClientBuilder::new().gzip(true).build().unwrap();
    let doc = client
        .get(&contest_url)
        .header("Accept-Language", "ja")
        .send()
        .await?
        .text()
        .await?;

    let pat = Pattern::new(
        r#"
        <ul class="nav nav-tabs">
            <ul class="dropdown-menu">
                <li><a href="{{pdf}}">PDF</a></li>
                <li><a href="{{youtube}}">YouTube</a></li>
            </ul>
        </ul>
    "#,
    )?;
    let result = pat.matches(&doc);
    for r in result {
        println!("{}", r["pdf"]);
        println!("{}", r["youtube"]);
    }
    Ok(())
}
