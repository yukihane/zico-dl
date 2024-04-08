use error::ZicoDlError;
use scraper::{Html, Selector};
use std::sync::OnceLock;

mod error;

pub async fn find_dl_target(url: &str) -> Result<String, ZicoDlError> {
    let body = reqwest::get(url).await?.text().await?;

    find_dl_target_html(&body)
}

// https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html
static SELECTOR: OnceLock<Selector> = OnceLock::new();
static SELECTOR_STRING: &str = ".trial_file a";

/// 製品ページのhtmlをパースし、体験版ファイルのURLを取得します。
fn find_dl_target_html(doc: &str) -> Result<String, ZicoDlError> {
    let selector = SELECTOR.get_or_init(|| Selector::parse(SELECTOR_STRING).unwrap());

    let parsed = Html::parse_document(doc);
    let link = parsed
        .select(&selector)
        .into_iter()
        .next()
        .ok_or(ZicoDlError::Content {
            msg: format!("There are no class expected, '{}'", SELECTOR_STRING),
        })?
        .attr("href")
        .ok_or(ZicoDlError::Content {
            msg: format!("No attribute, href."),
        })?;

    Ok(format!("https:{}", link))
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
    async fn find_dl_target_test() -> anyhow::Result<()> {
        let result =
            find_dl_target("https://www.dlsite.com/maniax/work/=/product_id/RJ292145.html").await?;

        assert_eq!(
            result,
            "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip"
        );

        Ok(())
    }

    #[test]
    fn find_dl_target_html() -> anyhow::Result<()> {
        let file = test_case!("dlsite/product_top.html");
        let html = fs::read_to_string(file)?;
        let result = super::find_dl_target_html(&html)?;

        assert_eq!(
            result,
            "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip"
        );

        Ok(())
    }
}
