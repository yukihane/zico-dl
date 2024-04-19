use anyhow::{anyhow, Context};
use error::ZicoDlError;
use futures_util::stream::StreamExt;
use pbr::ProgressBar;
use scraper::{Html, Selector};
use std::sync::OnceLock;
use tokio::io::AsyncWriteExt;
mod error;

// https://blog.foresta.me/posts/large-file-download/ を参考にした
/// ファイルをダウンロードします。
/// - `url` - ダウンロード対象。 ex: https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip
/// - `filepath` - ダウンロード場所。
pub async fn download_file(url: &str, filepath: &str) -> Result<(), ZicoDlError> {
    let client = reqwest::Client::new();
    let conent_length = get_content_length(&client, url).await?;

    let mut file = tokio::fs::File::create(filepath)
        .await
        .map_err(|_| ZicoDlError::Local)?;

    let mut pb = ProgressBar::new(conent_length);
    pb.set_units(pbr::Units::Bytes);
    pb.set_width(Some(100));

    let mut stream = client.get(url).send().await?.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = &chunk_result?;

        pb.add(chunk.len() as u64);

        file.write_all(&chunk)
            .await
            .map_err(|_| ZicoDlError::Local)?;
    }

    file.flush().await.map_err(|_| ZicoDlError::Local)?;

    Ok(())
}

async fn get_content_length(client: &reqwest::Client, url: &str) -> Result<u64, ZicoDlError> {
    let head_result = client.head(url).send().await?;
    let headers = head_result.headers();
    let content_length_header = headers
        .get("content-length")
        .ok_or(anyhow!("No content-length, {}", url))?;

    let content_length = content_length_header
        .to_str()
        .context("fail to str")?
        .parse::<u64>()
        .context("parse error")?;

    Ok(content_length)
}

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
    #[ignore]
    async fn download_file() -> anyhow::Result<()> {
        let url = "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip";
        let filepath = "download.zip";

        super::download_file(url, filepath).await?;

        Ok(())
    }

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
