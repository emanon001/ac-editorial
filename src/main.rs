use clap::{App, Arg};
use easy_scraper::Pattern;
use url::Url;

const ATCODER_BASE_URL: &str = "https://atcoder.jp";

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
    let editorial_url = format!("{}/contests/{}/editorial", ATCODER_BASE_URL, contest_id);
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
        <h3>{{title}}</h3>
        <ul>
            <li><a href="{{url}}">公式解説</a></li>
        </ul>
        "#,
    )?;
    let result = pat.matches(&doc);
    for r in result {
        let title = &r["title"];
        let url = normalize_url(&r["url"])?;
        if title == "コンテスト全体の解説" {
            println!("{}", url);
        } else {
            println!("{} {}", title, url);
        };
    }

    Ok(())
}

fn normalize_url(url_or_path: &str) -> Result<Url, url::ParseError> {
    Url::parse(url_or_path).or_else(|_| {
        // try join base URL & path
        Url::parse(ATCODER_BASE_URL)?.join(url_or_path)
    })
}
