use std::sync::OnceLock;

use scraper::{Html, Selector};

pub async fn find_dl_target(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;

    let trial_url = find_dl_target_html(&body);

    Ok(trial_url)
}

// https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html
static SELECTOR: OnceLock<Selector> = OnceLock::new();

/// 製品ページのhtmlをパースし、体験版ファイルのURLを取得します。
fn find_dl_target_html(doc: &str) -> String {
    let selector = SELECTOR.get_or_init(|| Selector::parse(".trial_file a").unwrap());

    let parsed = Html::parse_document(doc);
    let link = parsed
        .select(&selector)
        .into_iter()
        .next()
        .unwrap()
        .attr("href")
        .unwrap();

    format!("https:{}", link)
}

#[cfg(test)]
mod tests {

    // https://stackoverflow.com/a/74550371
    macro_rules! test_case {
        ($fname:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname) // assumes Linux ('/')!
        };
    }

    use std::fs;

    use super::*;

    #[tokio::test]
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
    #[ignore]
    async fn find_dl_target_test() {
        let result =
            find_dl_target("https://www.dlsite.com/maniax/work/=/product_id/RJ292145.html")
                .await
                .unwrap();
        assert_eq!(
            result,
            "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip"
        )
    }

    #[test]
    fn find_dl_target_html() {
        let file = test_case!("dlsite/product_top.html");
        let html = fs::read_to_string(file).unwrap();
        let result = super::find_dl_target_html(&html);

        assert_eq!(
            result,
            "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip"
        )
    }
}
