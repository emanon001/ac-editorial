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
    let editorial_url = format!("https://atcoder.jp/contests/{}/editorial", contest_id);
    let client = reqwest::ClientBuilder::new().gzip(true).build().unwrap();
    let doc = client
        .get(&editorial_url)
        .header("Accept-Language", "ja")
        .send()
        .await?
        .text()
        .await?;

    let pat = Pattern::new(
        r#"
        <h3>コンテスト全体の解説</h3>
        <ul>
            <li><a href="{{url}}">公式解説</a></li>
        </ul>
        "#,
    )?;
    let result = pat.matches(&doc);
    for r in result {
        println!("{}", r["url"]);
    }
    Ok(())
}
