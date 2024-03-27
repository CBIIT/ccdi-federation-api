The CCDI Data Federation API supports the querying of
federated pediatric cancer within the broader community. The goal of the API
is to support identification of pediatric cancer samples of interest via a
variety of query parameters.

### Definitions

**Authentication** is defined as being identified in any manner.
Authentication, by definition, requires prior registration of identifiable
characteristics. Typically, this is in the form of an account, though the
definition of authentication includes any condition under which you are
registered and identified (e.g., allowlisted via IP address).

**Authorization** is defined as permission explicitly granted or withheld from
an authenticated individual by a controlling entity based on a set of
non-authentication-based criteria (irrespective of the time period for which
access is granted or denied). Note that this definition of authorization always
requires prior authentication, so simply requiring authentication to gain
access to a resource is not considered authorization. For example, gaining
permission to a dataset via an explicit decision from a data access committee
is considered authorization while making data available after simply completing
a universally accessible account registration process is not.

**Primary Entities** are defined as classes of information for which the API
specification was created to share. In other words, the sharing of primary 
entities is the most important goal of the specification.

**Supporting Entities** are defined as classes of information that are necessary
to share alongside to make sense of primary entities. Sharing information on
these entities is not a top-level goal of the API specification.

### Security Requirements

All API endpoints must be served over HTTPS (port 443) with a certificate signed
by a recognized certificate authority. In particular, self-signed certificates
are not permitted. Further, while an API _may_ be available over HTTP (port 80),
HTTPS must always be available. We highly recommend servers redirect HTTP to
HTTPS rather than serve your API on two separate ports.

### Invalid Routes

All responses that do not match an endpoint below should return a Not Found 
(`404`) response. The body of this response should be the `responses.Errors` 
JSON object with one `responses.error.Kind` where the `Kind` matches the 
`InvalidRoute` error.

## Primary Entities

Primary entities represent information that this API specification was created
to share as a top-level goal. Primary entities have a common API surface and,
generally, will work relatively similar to one another within the specification.
All primary entities are referred to by their _identifier_, which is the
combination of (a) a namespace identifier pointing to the namespace that owns
this entity along with (b) a name for the entity.

The following entities are considered primary entities within the API
specification.

- Subjects
- Samples
- Files


In addition to merely existing in a common level of prominence within the API,
primary entities have a hierarchical structure following these rules.

- Subjects are the highest-level primary entity within the API specification.
- Samples **must** be associated with one and only one subject.
- Files **must** be associated with one or more samples.

### Subjects

TODO

### Samples

TODO

### Files

TODO

#### Accessing External Files

A **gateway** notifies end users of resources that exist outside of the API
along with the conditions under which those resources may or may not be
accessed. Gateways can be open access (open to anyone—even anonymously),
registered access (requires authentication but not authorization),
controlled access (requires both authentication and authorization), or
closed access (not available). Gateways do not, in and of themselves,
communicate the location of or mechanism(s) by which resources can be accessed.
Instead, gateways wrap `Link`s to communicate that information.

A **link** defines the mechanism for locating (referred to here as \"navigating
to\") an external resource. Links can be direct (for navigating to precisely the
requested resource(s)—no more and no less), approximate (a link which, when
followed, requires prior manual intervention in the form of instructions to
proceed to the otherwise immediately navigable desired resource(s)),
informational (a link that does not navigate to a desired resource directly but
can be followed to find out more information on how to access the desired
resource(s) out-of-band), or mailing (an email address to contact, accompanied
by instructions to access the desired resource(s)) in nature.
Again, a link does not, in and of itself, communicate the requirements to
access desired resources—it must be used in conjunction with a gateway to
communicate that information.

Put simply, a **link** tells you where you need to go to attempt to access a
desired resource and the **gateway** wrapping the link tells you what the
requirements for accessing the resource are once you get there. By separating
the concepts of the requirements to access a desired resource (gateways) from
the mechanism to access the desired resource (links), we create an expressive,
combinatorial system for capturing a broad spectrum of situations.

**Note:** a _closed_ gateway is special and does not include links. Instead, 
its purpose is to describe where a resource originated from and to communicate
that the resource is otherwise unavailable. Various closed gateway statuses are
provided to indicate if and when the resources will become available.

##### Examples

Below are some examples using pseudocode to illustrate these concepts. Note
that some fields have been left out of the definitions for brevity.

* A `Gateway::Controlled { Link::Exact { url: 
  \"https://example.com/files?sample=Sample001\" } }` communicates \"all of the
  resources you requested are available at 
  https://example.com/files?sample=Sample001, but be advised that the data found
  at that link is controlled access and requires authorization\".
* A `Gateway::Open { Link::MailTo { uri: \"mailto:data@example.com\" } }`
  communicates \"anyone can request the resource by emailing the provided email
  address—even if we haven't identified (authenticated) the individual
  requesting the data. In contrast, if the data provider required
  identification of the individual before sending the data (say, via a verified
  university email address), then a `Gateway::Registered` should be used
  instead. 
* A `Gateway::Registered { Link::Approximate { url: 
  \"https://example.com/data\", instructions: \"Filter data by ...\" } }`
  communicates \"the data is available to anyone who registers an account at
  https://example.com/data, but manual filtering (by following the provided
  instructions) is required to select the exact subset of desired data\".

## Supporting Entities

Supporting entities provide supporting information necessary to make sense of
the primary entities supported by the API. Supporting entities are not, in and
of themselves, a primary sharing goal for the API.

The following entities are considered supporting entities within the API
specification.

- Organizations
- Namespaces

### Organizations

Organizations are self-reported, non-authoritative descriptions of organizations
that are sharing data through an API endpoint. There is no formal definition or
criteria for what constitutes an organization in this context. Some examples of
what an organization might represent include (but are not limited to) 
for-profit companies, non-profit organizations, consortiums, informal bodies, or
any combination of these concepts.

### Namespaces

Namespaces represent top-level governance groupings of primary entities within
the CCDI Federation API. Each namespace is owned by an existing
organization entity, contains information about the governance unit, and 
provides information on how to contact the body that governs this namespace.

### Assigning Organizations and Namespaces

When assigning namespaces within a source server, one should consider making a
namespace entity for each grouping of primary entities that are governed under
a common model. 

Here are some common situations followed by instructive examples of how you
partition primary entities to a set of namespaces under that situation:

- If all of the primary entities within your source server are governed by a
  singular governing body (say, a single data access committee), then you may
  only need one namespace for all of the primaries entities within your server.
- If you have multiple data access committees governing different groupings of
  primary entities from the same institution, you should create multiple 
  namespaces that are backed by a common organization.
- If your server serves data from various governing bodies spread across
  multiple, independent organizations, you should create multiple namespaces
  backed by multiple organizations.
