#[cfg(test)]
mod dependencies_test {
    use reqwest::header::USER_AGENT;
    use serde::Deserialize;
    use std::env;

    #[derive(Deserialize)]
    struct GitHubResponse {
        tag_name: String,
    }

    #[test]
    fn using_latest_tailwindcss_version() {
        let key = "VIGILANT_ARCHIVE_USER_AGENT";
        let message = &*format!(
            "Please set the environment variable {} to run dependency tests",
            key
        );
        let user_agent = env::var(key).expect(message);
        let response: GitHubResponse = reqwest::blocking::Client::new()
            .get("https://api.github.com/repos/tailwindlabs/tailwindcss/releases/latest")
            .header(USER_AGENT, user_agent)
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!("v3.3.6", response.tag_name);
    }
}
