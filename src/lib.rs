pub fn find_dl_target(url: &str) -> String {
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_dl_target_test() {
        let result =
            find_dl_target("https://www.dlsite.com/maniax/work/=/product_id/RJ292145.html");
        assert_eq!(
            result,
            "https://trial.dlsite.com/doujin/RJ293000/RJ292145_trial.zip"
        )
    }
}
