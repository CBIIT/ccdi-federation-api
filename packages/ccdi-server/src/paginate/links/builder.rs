//! A builder for [`Links`].

use std::collections::HashMap;
use std::num::NonZeroUsize;

use url::Url;

use crate::paginate::links::Link;
use crate::paginate::links::Links;
use crate::paginate::links::Relationship;

/// A parse error related to a [`Builder`].
#[derive(Debug)]
pub enum ParseError {
    /// A URL parse error.
    UrlParseError(url::ParseError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // NOTE: since this is already a parse error, we will pass it
            // through invisibly.
            ParseError::UrlParseError(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// An error related to a [`Builder`].
#[derive(Debug)]
pub enum Error {
    /// A parse error.
    ParseError(ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(err) => write!(f, "parse error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

/// Details regarding a page.
#[derive(Debug)]
pub struct Page {
    /// The index of this page.
    index: NonZeroUsize,

    /// The requested number of entities within this page.
    /// The actual number may be fewer if this is the last page.
    page_size: NonZeroUsize,
}

impl Page {
    /// Creates a new [`Page`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use std::num::NonZeroUsize;
    ///
    /// let page = server::paginate::links::builder::Page::new(
    ///     NonZeroUsize::try_from(1).unwrap(),
    ///     NonZeroUsize::try_from(10).unwrap(),
    /// );
    /// ```
    pub fn new(page: NonZeroUsize, per_page: NonZeroUsize) -> Self {
        Self {
            index: page,
            page_size: per_page,
        }
    }
}

/// A builder for [`Link`]s.
#[derive(Debug)]
pub struct Builder<'a, T> {
    base: Url,
    current_page: NonZeroUsize,
    pages: Vec<&'a [T]>,
    links: HashMap<Relationship, Page>,
}

impl<'a, T> Builder<'a, T> {
    /// Attempts to create a new [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZeroUsize;
    ///
    /// use ccdi_server as server;
    ///
    /// use server::paginate::links::Builder;
    ///
    /// let page = [String::from("Hello, world!")];
    /// let pages = vec![&page[..]];
    ///
    /// let builder = Builder::<String>::try_new(
    ///     "https://example.com",
    ///     NonZeroUsize::try_from(1).unwrap(),
    ///     pages,
    /// )?;
    ///
    /// Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_new(base: &str, current_page: NonZeroUsize, pages: Vec<&'a [T]>) -> Result<Self> {
        let base = base
            .parse::<Url>()
            .map_err(ParseError::UrlParseError)
            .map_err(Error::ParseError)?;

        Ok(Self {
            base,
            current_page,
            pages,
            links: Default::default(),
        })
    }

    /// Attempts to add a new link for the specified [`Relationship`].
    ///
    /// - Links representing [Relationship::First] and [Relationship::Last] will
    ///   always be added.
    /// - Links representing [Relationship::Prev] will be added when there is
    ///   more than one page and the current page is not the first page.
    /// - Links representing [Relationship::Next] will be added when there is
    ///   more than one page and the current page is not the last page.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZeroUsize;
    ///
    /// use ccdi_server as server;
    ///
    /// use server::paginate::links::Builder;
    /// use server::paginate::links::Relationship;
    ///
    /// let page = [String::from("Hello, world!")];
    /// let pages = vec![&page[..]];
    ///
    /// let builder = Builder::<String>::try_new(
    ///     "https://example.com",
    ///     NonZeroUsize::try_from(1).unwrap(),
    ///     pages,
    /// )?;
    ///
    /// builder.insert_link(Relationship::First);
    ///
    /// Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn insert_link(mut self, rel: Relationship) -> Self {
        match rel {
            Relationship::First => {
                // SAFETY: we check to ensure that `self.pages` is not empty at
                // [`Builder`] creation time. As such, this call to `first()`
                // will always unwrap, as at least one page exists.
                let contents = self.pages.first().unwrap();

                self.links.insert(
                    rel,
                    Page::new(
                        // SAFETY: this is the first page, so the page value should
                        // always be `1`. That will always unwrap a [`NonZeroUsize`].
                        NonZeroUsize::try_from(1).unwrap(),
                        // SAFETY: similarly, because we know that the contents
                        // are not empty, this will always have at least one
                        // element (and, thus, will unwrap).
                        NonZeroUsize::try_from(contents.len()).unwrap(),
                    ),
                );
            }
            Relationship::Prev => {
                // A link for [`Relationship::Prev`] is only added if (a) there
                // is more than one page and (b) the current page is not the
                // first page.
                if self.pages.len() > 1 && self.current_page.get() != 1 {
                    let prev_page = self.current_page.get() - 1;

                    // SAFETY: we just checked to make sure that there is (a)
                    // more than one page and (b) we are not currently pointed
                    // at page one. As such, there will always be a prev page
                    // that we can unwrap.
                    //
                    // Recall that the index starts at 0, so to get the prev
                    // page, we have to get the `prev_page - 1` nth item.
                    let contents = self.pages.get(prev_page - 1).unwrap();

                    self.links.insert(
                        rel,
                        Page::new(
                            // SAFETY: we just checked that current_page is > 1, so
                            // prev_page will be >= 1 thus always unwrap a [`NonZeroUsize`].
                            NonZeroUsize::try_from(prev_page).unwrap(),
                            // SAFETY: similarly, because we know that the contents
                            // are not empty, this will always have at least one
                            // element (and, thus, will unwrap).
                            NonZeroUsize::try_from(contents.len()).unwrap(),
                        ),
                    );
                }
            }
            Relationship::Next => {
                // A link for [`Relationship::Next`] is only added if (a) there
                // is more than one page and (b) the current page is not the
                // last page.
                if self.pages.len() > 1 && self.current_page.get() != self.pages.len() {
                    let next_page = self.current_page.get() + 1;

                    // We retrieve contents here only to calculate a per_page value.
                    // However, we cannot use self.pages.get(next_page - 1) for this purpose:
                    // if next_page is the last page, and the overall number of entities isn't
                    // evenly divisible by the per_page value, then the count of entities
                    // on that page will be less than the requested per_page value.
                    // Instead, get the length of the contents of the first page, which will
                    // always be a full page unless per_page > total count of entities.
                    // (In which case it's ok to change per_page to the total count.)
                    //
                    // SAFETY: we check to ensure that `self.pages` is not empty at
                    // [`Builder`] creation time. As such, this call to `first()`
                    // will always unwrap, as at least one page exists.
                    let first_page_contents = self.pages.first().unwrap();

                    self.links.insert(
                        rel,
                        Page::new(
                            // SAFETY: next_page is a NonZeroUsize plus 1.
                            // This could fail to unrap if it goes past usize::MAX.
                            // In its practical context as a page number, this is unlikely.
                            NonZeroUsize::try_from(next_page).unwrap(),
                            // SAFETY: because there is more than one page, there must be at
                            // least one element on the first page, so this will unwrap.
                            NonZeroUsize::try_from(first_page_contents.len()).unwrap(),
                        ),
                    );
                }
            }
            Relationship::Last => {
                let last_page = self.pages.len();

                // We retrieve contents here only to calculate a per_page value.
                // However, we cannot use self.pages.last.unwrap() for this purpose:
                // when we are on the last page and the overall number of entities isn't
                // evenly divisible by the per_page value, then the count of entities
                // on that page will be less than the requested per_page value.
                // Instead, get the length of the contents of the first page, which will
                // always be a full page unless per_page > total count of entities.
                // (In which case it's ok to change per_page to the total count.)
                //
                // SAFETY: we check to ensure that `self.pages` is not empty at
                // [`Builder`] creation time. As such, this call to `first()`
                // will always unwrap, as at least one page exists.
                let first_page_contents = self.pages.first().unwrap();

                self.links.insert(
                    rel,
                    Page::new(
                        // SAFETY: because we know that the contents are not
                        // empty, the last page will be, at least, one. Thus,
                        // this will always unwrap.
                        NonZeroUsize::try_from(last_page).unwrap(),
                        // SAFETY: because there is more than one page, there must be at
                        // least one element on the first page, so this will unwrap.
                        NonZeroUsize::try_from(first_page_contents.len()).unwrap(),
                    ),
                );
            }
        }

        self
    }

    /// Consumes `self` to build an immutable [`Links`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZeroUsize;
    ///
    /// use ccdi_server as server;
    ///
    /// use server::paginate::links::Builder;
    /// use server::paginate::links::Relationship;
    ///
    /// let page = [String::from("Hello, world!")];
    /// let pages = vec![&page[..]];
    ///
    /// let links = Builder::<String>::try_new(
    ///     "https://example.com",
    ///     NonZeroUsize::try_from(1).unwrap(),
    ///     pages,
    /// )?
    /// .insert_link(Relationship::First)
    /// .build();
    ///
    /// assert_eq!(
    ///     links.to_string(),
    ///     String::from("<https://example.com/?page=1&per_page=1>; rel=\"first\"")
    /// );
    ///
    /// Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn build(self) -> Links {
        let base_url = self.base.to_string();

        Links::from(
            self.links
                .into_iter()
                .map(|(relationship, location)| {
                    Link::new(
                        relationship,
                        // SAFETY: we already checked to ensure the base URL is a valid
                        // URL when we created this builder. Thus, that cannot be a
                        // problem at this point. Further, these parameters are hand
                        // crafted such that they will not fail. As such, this URL
                        // parsing will always unwrap.
                        Url::parse_with_params(
                            &base_url,
                            &[
                                ("page", location.index.to_string()),
                                ("per_page", location.page_size.to_string()),
                            ],
                        )
                        .unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }
}
