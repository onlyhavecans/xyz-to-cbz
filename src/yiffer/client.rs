use derive_builder::Builder;
use fantoccini::{ClientBuilder, Locator};
use url::Url;

#[derive(Debug, Builder, Clone)]
#[builder(setter(into), default)]
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
    pub fn new() -> Self {
        Self {
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
        let c = ClientBuilder::rustls().connect(GECKODRIVER).await?;
        c.goto(url.as_str()).await?;

        let title = Locator::Css("h1.loadedComicHeader");

        let t = c.wait().for_element(title).await?;
        println!("{:?}", t);
        let text = c.source().await?;
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
