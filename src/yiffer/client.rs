use fantoccini::{ClientBuilder, Locator};
use url::Url;

#[derive(Debug, Clone)]
pub struct YifferClient {
    base_url: String,
}

const BASE_URL: &str = "https://yiffer.xyz/";
const GECKODRIVER: &str = "http://localhost:4444";

impl Default for YifferClient {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.into(),
        }
    }
}

impl YifferClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn comic_url(&self, name: &str) -> Result<Url, url::ParseError> {
        let base = Url::parse(&self.base_url)?;
        base.join(name)
    }

    pub async fn comic_page(&self, name: &str) -> anyhow::Result<String> {
        let url = self.comic_url(name)?;
        let c = ClientBuilder::rustls().connect(GECKODRIVER).await?;
        c.goto(url.as_str()).await?;

        let title = Locator::Css("h1.loadedComicHeader");

        let _ = c.wait().for_element(title).await?;
        let text = c.source().await?;
        c.close_window().await?;
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
        let body = std::fs::read_to_string("test/Kissy Cousin - Yiffer.html").unwrap();
        let response = ResponseTemplate::new(200).set_body_raw(body, "text/html");
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/hello"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let uri = mock_server.uri();
        let client = YifferClient::new(uri);
        let document = client.comic_page("hello").await.unwrap();

        assert!(document.contains("Kissy Cousin page 42"));
    }
}
