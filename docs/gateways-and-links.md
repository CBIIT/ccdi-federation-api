# Gateways and Links

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

## Examples

Below are some examples using pseudocode to illustrate these concepts. Note
that some fields have been left out of the definitions for brevity.

- A `Gateway::Controlled { Link::Exact { url:
\"https://example.com/files?sample=Sample001\" } }` communicates \"all
  resources you requested are available at
  https://example.com/files?sample=Sample001, but be advised that the data
  found at that link is controlled access and requires authorization\".
- A `Gateway::Open { Link::MailTo { uri: \"mailto:data@example.com\" } }`
  communicates \"anyone can request the resource by emailing the provided email
  address—even if the requesting individual has not identified themselves via
  authentication before requesting the data. In contrast, if the data provider
  required identification of the individual before sending the data (say, via a
  verified university email address), then a `Gateway::Registered` should be
  used instead.
- A `Gateway::Registered { Link::Approximate { url:
\"https://example.com/data\", instructions: \"Filter data by ...\" } }`
  communicates \"the data is available to anyone who registers an account at
  https://example.com/data, but manual filtering (by following the provided
  instructions) is required to select the exact subset of desired data\".
