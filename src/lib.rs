pub async fn find_dl_target(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use std::future::IntoFuture;

    use super::*;

    #[tokio::test]
    async fn find_dl_target_test() {
        let result =
            find_dl_target("https://www.dlsite.com/maniax/work/=/product_id/RJ292145.html");
        assert_eq!(
            result.into_future().await.unwrap(),
            "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip"
        )
    }
}
