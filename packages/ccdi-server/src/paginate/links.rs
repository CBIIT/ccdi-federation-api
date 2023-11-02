//! Paginated links.

pub mod builder;

pub use builder::Builder;
use itertools::Itertools;
use url::Url;

mod relationship;

pub use relationship::Relationship;

/// A link for inclusion in the `link` response header.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Link {
    rel: Relationship,
    url: Url,
}

impl Link {
    /// Creates a new [`Link`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::paginate::links::Link;
    /// use server::paginate::links::Relationship;
    /// use url::Url;
    ///
    /// let url = "https://example.com?page=1&per_page=10".parse::<Url>()?;
    /// let link = Link::new(Relationship::First, url);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(rel: Relationship, url: Url) -> Self {
        Self { rel, url }
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>; rel=\"{}\"", self.url, self.rel)
    }
}

/// A set of aggregated links to include in the `link` response header.
#[derive(Debug)]
pub struct Links(Vec<Link>);

impl From<Vec<Link>> for Links {
    fn from(value: Vec<Link>) -> Self {
        Links(value)
    }
}

impl std::fmt::Display for Links {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().sorted().join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_displays_links() {
        let url = "https://example.com".parse::<Url>().unwrap();
        let link = Link::new(Relationship::First, url);
        assert_eq!(
            link.to_string(),
            String::from("<https://example.com/>; rel=\"first\"")
        );
    }
}
