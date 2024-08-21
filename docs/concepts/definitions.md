# Definitions

- **Authentication** is defined as identifying oneself in any manner.
  Authentication, by definition, requires prior registration of identifiable
  characteristics. Typically, this is in the form of an account, though the
  definition of authentication includes any condition under which you are
  registered and identified (for example, allowlisted via IP address).

- **Authorization** is defined as a permission explicitly granted or withheld
  from an authenticated individual by a controlling entity based on a set of
  non-authentication-based criteria (irrespective of the period of time for
  which access is granted or denied). Note that this definition of
  authorization always requires prior authentication, so simply requiring
  authentication to gain access to a resource is not considered authorization.
  For example, gaining permission to a dataset via an explicit decision from a
  data access committee is considered authorization while making data available
  after simply completing a universally accessible account registration process
  is not.

- **Entities** are defined as objects that share a particular kind of
  information through the API specification. There are two types of entities:
  _Primary_ and _Supporting_ entities, and these differences between these two
  types of entities are described below.

  - **Primary Entities** are defined as Entities which are inherently valuable
    and for which this API specification was designed to share. Sharing these
    entities effectively with the community is the top-level goal for the
    specification.

  - **Supporting Entities** are defined Entities that are shared to aid in
    interpreting and making sense of primary entities shared through the API.
