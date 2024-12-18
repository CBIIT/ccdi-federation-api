/// Relationship of a [`Link`](super::Link) to the current page.
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Relationship {
    /// The first page in a paginated result set.
    First,

    /// The previous page in a paginated result set.
    Prev,

    /// The next page in a paginated result set.
    Next,

    /// The last page in a paginated result set.
    Last,
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relationship::First => write!(f, "first"),
            Relationship::Prev => write!(f, "prev"),
            Relationship::Next => write!(f, "next"),
            Relationship::Last => write!(f, "last"),
        }
    }
}
