use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A uniform resource locator (URL) according to the [URL
/// Standard](https://url.spec.whatwg.org/).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::Url, value_type = String)]
pub struct Url(url::Url);

impl std::ops::Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<url::Url> for Url {
    fn from(value: url::Url) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Url {
    type Error = url::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = url::Url::parse(value)?;
        Ok(Self(url))
    }
}
