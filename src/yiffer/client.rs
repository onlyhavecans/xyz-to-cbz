use derive_builder::Builder;
use reqwest::Client;
use url::Url;

#[derive(Debug, Builder, Clone)]
#[builder(setter(into), default)]
pub struct YifferClient {
    http_client: Client,
    base_url: String,
}

const BASE_URL: &str = "https://yiffer.xyz/";

impl Default for YifferClient {
    fn default() -> Self {
        Self {
            http_client: Client::new(),
            base_url: BASE_URL.into(),
        }
    }
}

impl YifferClient {
    pub fn new(client: Client) -> Self {
        Self {
            http_client: client,
            ..Default::default()
        }
    }

    pub fn builder() -> YifferClientBuilder {
        YifferClientBuilder::default()
    }

    pub fn comic_url(&self, name: &str) -> Result<Url, url::ParseError> {
        let base = Url::parse(&self.base_url)?;
        base.join(name)
    }

    pub async fn comic_page(&self, name: &str) -> anyhow::Result<String> {
        let url = self.comic_url(name)?;
        let response = self.http_client.get(url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}

#[cfg(test)]
mod test {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[test]
    fn comic_url() {
        let client = YifferClient::default();
        let url = client.comic_url("test%20page").unwrap();
        let desired = Url::parse("https://yiffer.xyz/test%20page").unwrap();
        assert_eq!(desired, url);

        let url = client.comic_url("test page").unwrap();
        assert_eq!(desired, url);
    }

    #[tokio::test]
    async fn request_page() {
        let body = std::fs::read_to_string("test/Blueberry Jam - Yiffer.html").unwrap();

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/hello"))
            .respond_with(ResponseTemplate::new(200).set_body_string(&body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let uri = mock_server.uri();
        let client = YifferClient::builder().base_url(uri).build().unwrap();
        let document = client.comic_page("hello").await.unwrap();
        assert_eq!(document, body);
    }
}
