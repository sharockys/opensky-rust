pub mod client {
    use log::warn;

    pub struct OpenSkyClient {
        client: reqwest::Client,
    }

    impl OpenSkyClient {
        pub fn new() -> Self {
            OpenSkyClient {
                client: reqwest::Client::new(),
            }
        }

        pub async fn states_all(&self) -> Result<String, reqwest::Error> {
            let url = "https://opensky-network.org/api/states/all";
            let response: reqwest::Response = self.client.get(url).send().await?;
            // warn 403 for rate limit
            if response.status().as_u16() == 403 {
                warn!("Warning! Rate limit reached! waiting for 10 seconds");
                // wait for 10 seconds
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
            let body = response.text().await?;
            Ok(body)
        }
    }
}
