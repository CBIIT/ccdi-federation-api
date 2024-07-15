## Overview

The Childhood Cancer Data Initiative (CCDI) Data Federation API supports the
querying of federated pediatric cancer within the broader community. The goal
of the API is to support identification of pediatric cancer samples of interest
via a variety of query parameters.

### Definitions

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

### Security requirements

All API endpoints must be served over HTTPS (port 443) with a certificate
signed by a recognized certificate authority. In particular, self-signed
certificates are not permitted. Further, while an API _may_ be available over
HTTP (port 80), HTTPS must always be available. The authors highly recommend
servers redirect HTTP to HTTPS rather than serve your API on two separate
ports.

### Invalid routes

All responses that do not match an endpoint below should return a Not Found
(`404`) response. The body of this response should be the `responses.Errors`
JSON object with one `responses.error.Kind` where the `Kind` matches the
`InvalidRoute` error.

### Incomplete Data and Nullity

_Note: strings are explicitly enclosed in double quotes (`"`) to distinguish
their string values from `null` in this section._

Metadata elements within this specification are harmonized to [Common Data
Elements](https://datascience.cancer.gov/resources/metadata) (CDEs) cataloged
at the [Cancer Data Standards Registry and
Repository](https://cadsr.cancer.gov) (caDSR). These CDEs are drafted by a
diverse set of stakeholders for a variety of purposes and, as such, they lack a
cross-CDE mechanism for the expressing the spectrum of incompleteness or
confidence that accompany expressing data elements.

To illustrate this concept, consider that the CDE selected to represent [vital
status](https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#vital_status)
for subjects is relatively comprehensive in its permissible values (`"Not
reported"`, `"Unknown"`, `"Unspecified"`), whereas the CDE selected to
represent [tumor
morphology](https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#tumor_tissue_morphology)
for samples has no such equivalent facilities to express an unknown or
incomplete value. Because uniformity amongst reporting metadata elements is
desirable to simplify parsing, a mechanism for explicit omission of information
is needed as a fallback across all data elements.

`null` values, as present in this specification, represent a lack of assertion
(or, equivalently, a "decline to comment") by the server. This value should not
be construed to communicate _any_ content about the field it represents—either
in affirmation or dissent. Instead, they should be interpreted simply as an
explicit omission of data that can be updated at any time if new information is
made available to the source server.

This is in stark contrast to explicit values of `"Unknown"` or `"Not
Reported"`, which often have explicit meaning in the context of the scope of
the CDE. Sometimes, these permissible values (particularly the `"Not Reported"`
value) venture into overlapping territory with the aforementioned definition of
`null` values. This introduces potential ambiguity when deciding which value to
use when representing nullity in a server's responses (or, conversely, how to
interpret these different representations of nullity when retrieving data from
multiple servers).

Use the following guidelines when deciding when to use `"Unknown"`/`"Not
Reported"`/`"Unavailable"`, when to use `null`, and how to interpret the values
in your API calls.

- First (and most importantly), make sure you carefully read and understand the
  context of the CDE by visiting the [caDSR](https://cadsr.cancer.gov) (or the
  [wiki pages](https://github.com/CBIIT/ccdi-federation-api/wiki)) and
  reviewing the CDE's "Definition." For example, a participant self-reporting a
  value of `"Unknown"` to a demographics question is _not_ the same as an
  investigator indicating `"Unknown"` for a value that is simply missing from
  their records. **Ultimately, all CDEs and their respective permissible values
  should be interpreted strictly in that context**.
- If the CDE provides a permissible value that covers the particular flavor of
  incomplete or unknown information in your case, **use the explicit
  permissible value assigned by the CDE**.
- If there is no permissible value that covers the particular flavor of
  incomplete or unknown information in your case, **use the `null` value to
  indicate that the server is explicitly declining to comment**. The authors
  recommend that you note why data is missing in the associated
  `/metadata/*/fields` entry to increase the likelihood that users interpret
  your values correctly. Additionally, please be sure to check the
  specification to ensure that a value of `null` is actually allowed!
- Interpreting equivalence of values such as `"Unknown"`, `"Not Reported"`,
  `"Unavailable"`, and `null` is a context-specific decision based on (a) the
  definition of the CDE and (b) the context in which you're applying the
  information. If the appropriate course of action for your situation is not
  immediately apparent, we ask that you file a [an API question on the GitHub
  Discussions
  page](https://github.com/CBIIT/ccdi-federation-api/discussions/categories/api-questions)
  so the community can come together to help answer your question.

## Primary entities

Primary entities represent information that this API specification was created
to share as a top-level goal. Primary entities have a common API surface and,
generally, will work relatively similar to one another within the
specification. All primary entities are referred to by their _identifier_,
which is the combination of (a) a namespace identifier pointing to the
namespace that owns this entity along with (b) a name for the entity.

The following entities are considered primary entities within the API
specification.

- Subjects
- Samples
- Files

Beyond merely existing in a common level of prominence within the API, primary
entities have a hierarchical structure following these rules.

- Subjects are the highest-level primary entity within the API specification.
- Samples **must** be associated with one and only one subject.
- Files **must** be associated with one or more samples.

#### Accessing external files

A **gateway** notifies end users of resources that exist outside of the API
along with the conditions under which those resources may or may not be
accessed. Gateways can be open access (open to anyone—even anonymously),
registered access (requires authentication but not authorization), controlled
access (requires both authentication and authorization), or closed access (not
available). Gateways do not, in and of themselves, communicate the location of
or mechanisms by which resources can be accessed. Instead, gateways wrap
`Link`s to communicate that information.

A **link** defines the mechanism for locating (referred to here as \"navigating
to\") an external resource. Links can be direct (for navigating to precisely
the requested resources—no more and no less), approximate (a link which, when
followed, requires prior manual intervention in the form of instructions to
proceed to the otherwise immediately navigable desired resources),
informational (a link that does not navigate to a desired resource directly but
can be followed to find out more information on how to access the desired
resources out-of-band), or mailing (an email address to contact, accompanied by
instructions to access the desired resources) in nature. Again, a link does
not, in and of itself, communicate the requirements to access desired
resources—it must be used in conjunction with a gateway to communicate that
information.

Put simply, a **link** tells you where you need to go to attempt to access a
desired resource, and the **gateway** wrapping the link tells you what the
requirements for accessing the resource are once you get there. By separating
the concepts of the requirements to access a desired resource (gateways) from
the mechanism to access the desired resource (links), the specification
provides an expressive, combinatorial system for capturing a broad spectrum of
situations.

**Note:** a _closed_ gateway is special and does not include links. Instead,
its purpose is to describe where a resource originated from and to communicate
that the resource is otherwise unavailable. Various closed gateway statuses are
provided to indicate if and when the resources will become available.

##### Examples

Below are some examples using pseudocode to illustrate these concepts. Note
that some fields have been left out of the definitions for brevity.

* A `Gateway::Controlled { Link::Exact { url:
  \"https://example.com/files?sample=Sample001\" } }` communicates \"all
  resources you requested are available at
  https://example.com/files?sample=Sample001, but be advised that the data
  found at that link is controlled access and requires authorization\".
* A `Gateway::Open { Link::MailTo { uri: \"mailto:data@example.com\" } }`
  communicates \"anyone can request the resource by emailing the provided email
  address—even if the requesting individual has not identified themselves via
  authentication before requesting the data. In contrast, if the data provider
  required identification of the individual before sending the data (say, via a
  verified university email address), then a `Gateway::Registered` should be
  used instead.
* A `Gateway::Registered { Link::Approximate { url:
  \"https://example.com/data\", instructions: \"Filter data by ...\" } }`
  communicates \"the data is available to anyone who registers an account at
  https://example.com/data, but manual filtering (by following the provided
  instructions) is required to select the exact subset of desired data\".

## Supporting entities

Supporting entities provide supporting information necessary to make sense of
the primary entities supported by the API. Supporting entities are not, in and
of themselves, a primary sharing goal for the API.

The following entities are considered supporting entities within the API
specification.

- Organizations
- Namespaces

### Organizations

Organizations are self-reported, non-authoritative descriptions of
organizations that are sharing data through an API endpoint. There is no formal
definition or criteria for what constitutes an organization in this context.
Some examples of what an organization might represent include (but are not
limited to) for-profit companies, non-profit organizations, consortiums,
informal bodies, or any combination of these concepts.

### Namespaces

Namespaces represent top-level governance groupings of primary entities within
the CCDI Federation API. Each namespace is owned by an existing organization
entity, contains information about the governance unit, and provides
information on how to contact the body that governs this namespace.

### Assigning organizations and namespaces

When assigning namespaces within a source server, one should consider making a
namespace entity for each grouping of primary entities that are governed under
a common model.

Here are some common situations followed by instructive examples of how you
partition primary entities to a set of namespaces under that situation:

- If all primary entities within your source server are governed by a singular
  governing body (say, a single data access committee), then you may only need
  one namespace for all primaries entities within your server.
- If you have multiple data access committees governing different groupings of
  primary entities from the same institution, you should create multiple
  namespaces that are backed by a common organization.
- If your server serves data from various governing bodies spread across
  multiple, independent organizations, you should create multiple namespaces
  backed by multiple organizations.

## Metadata

### Interpreting metadata assignments

Metadata shared through this API is expected to be collected and harmonized
through a variety of mechanisms by individuals with a wide range of
backgrounds. In certain cases, communicating the process by which metadata was
collected and harmonized will be critical to correctly interpreting the
data—particularly in metadata fields where the permissible values are rapidly
evolving (for example, diagnosis ontologies).

Within this API, there are two primary ways you can learn about harmonized
metadata fields: (a) via the appropriate `/metadata/fields/<entity>` endpoint
and (b) through the `details` key within a harmonized metadata object.

- **For information concerning harmonized metadata values where the information
  is universally true for all value assignments provided by the server**, the
  information should be present in the `/metadata/fields/<entity>` endpoint
  response.
- **For information concerning harmonized metadata values where the information
  is specific to a particular value assignment (or subset of value assignments)
  provided by the server**, the information should be present in the
  `details` key within the harmonized metadata object.

In general, when information _can_ be included in the
`/metadata/fields/<entity>` response instead of embedded in `details` keys, it
should be, as that consolidates the information without repeating it for every
entity. That said, sometimes it is not possible to make blanket statements
about all harmonized values for a particular metadata field: when the data
_cannot_ be included in the `/metadata/fields/<entity>` responses, it should be
reported under the correct facility within the `details` key of the individual
harmonized value objects.

#### An illustrative example

Consider an example where there are exactly two datasets, **Dataset A** and
**Dataset B**, that are both served by a single API server.

**When to store information in `/metadata/fields/<entity>`**

Assume that, for both datasets, the team running the API server received
[age_at_collection](https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#age_at_collection)
information _exactly_ as the harmonized data value expects and no further
curation was required. Here, the metadata values were handled the exact same
way for all datasets provided by the API server. In these cases, any details
regarding the harmonization process should be provided via the
`/metadata/fields/<entity>` endpoint rather than duplicating the information in
every metadata assignment object in the response.

**When to store information in the `details` key of harmonized metadata values**

Conversely, consider a scenario where the two datasets collected and harmonized
the values for [tissue
type](https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#tissue_type)
in different ways:

- For **Dataset A**, assume the team running the API server received values
  aligned to the harmonized value list, and no further curation was required.
- For **Dataset B**, assume the team running the API server had to manually
  curate the mappings between the data they received to the harmonized values.

In this case, no universal statement can be made about the process by which the
data was harmonized for the tissue type field. As such, no universal statement
can be made about the harmonization process in `/metadata/fields/<entity>`.
Instead, details regarding the harmonization process for these values should be
included directly in the `details` key of each of the metadata value assignment
objects.

#### A final note on harmonization details

Note that an API server is not _required_ to provide harmonization details for
any of the harmonized metadata it provides. This mechanism is simply available
to include information that is deemed contextually relevant at the discretion of
the curation team.

### CCDI Data Federation Resource

To access the CCDI Data Federation Resource, please click [here](https://cbiit.github.io/ccdi-federation-api-aggregation/).
