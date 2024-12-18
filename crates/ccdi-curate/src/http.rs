//! HTTP facilities.

use eyre::bail;
use eyre::Context;
use eyre::Result;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::Response;
use reqwest::header::LOCATION;
use reqwest::redirect::Policy;
use tracing::info;

/// An HTTP client.
///
/// Note that requests are not automatically redirected in this client, as we
/// often need to track the URLs that are redirected to determine the version of
/// various files.
pub struct Client(ReqwestClient);

impl Default for Client {
    fn default() -> Self {
        let client = ReqwestClient::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION"),
            ))
            .redirect(Policy::none())
            .build()
            // SAFETY: this should always unwrap.
            .expect("reqwest client to build");

        Self(client)
    }
}

impl Client {
    /// Follows a URL using HEAD requests and tracking each URL along the way.
    pub fn follow_via_head(&self, url: impl Into<String>) -> Result<Vec<String>> {
        let url = url.into();
        info!("starting at {}", url);

        let mut results = vec![url];

        loop {
            // SAFETY: we always add the first argument to the Vec, so this will
            // always unwrap.
            let last_url = results.last().unwrap();
            let response = self.0.get(last_url).send()?;

            if !response.status().is_success() && !response.status().is_redirection() {
                bail!(
                    "failed to retrieve URL with status {}: {}",
                    response.status().as_str(),
                    last_url
                );
            }

            if let Some(redirect) = response.headers().get(LOCATION) {
                // SAFETY: this should never fail to unwrap for our use cases.
                let redirect = redirect.to_str().unwrap().to_string();
                info!("redirecting to {}", redirect);
                results.push(redirect)
            } else {
                info!("finished at {}", last_url);
                break;
            }
        }

        Ok(results)
    }

    /// Performs a GET request on a URL.
    pub fn get(&self, url: impl AsRef<str>) -> Result<Response> {
        let url = url.as_ref();
        info!("sending a GET request to {}", url);

        let response = self
            .0
            .get(url)
            .send()
            .with_context(|| format!("sending a GET request to {}", url))?;

        if !response.status().is_success() {
            bail!("failed to GET the URL {}", url);
        }

        Ok(response)
    }
}
