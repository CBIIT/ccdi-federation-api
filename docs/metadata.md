# Metadata

The CCDI Federation API specifies a rich set of metadata elements for each of
the entities shared through the specification. You can see a full list of
metadata shared for each entity in [the GitHub
wiki](https://github.com/CBIIT/ccdi-federation-api/wiki).

## Interpreting metadata assignments

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

### An illustrative example

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

### A final note on harmonization details

Note that an API server is not _required_ to provide harmonization details for
any of the harmonized metadata it provides. This mechanism is simply available
to include information that is deemed contextually relevant at the discretion of
the curation team.
