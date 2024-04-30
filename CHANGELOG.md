# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Adds both local and global gateway definitions to subjects and samples. With
  this change, subjects, samples, and files now have the same top-level keys in
  their responses ([link to
  discussion](https://github.com/CBIIT/ccdi-federation-api/discussions/79),
  [#95](https://github.com/CBIIT/ccdi-federation-api/pull/95)).

### Revised

- Removes namespace-partitioned count by results ([link to
  dicussion](https://github.com/CBIIT/ccdi-federation-api/discussions/96),
  [#97](https://github.com/CBIIT/ccdi-federation-api/pull/97)).
- Updates diagnosis to support multiple values
  ([#99](https://github.com/CBIIT/ccdi-federation-api/pull/99)).

### Fixed

- Includes diagnosis in sample metadata field descriptions.

## [v1.0.0-rc.1] — 04-26-2024

### Added

- Adds an endpoint to show an individual file ([link to
  card](https://github.com/orgs/CBIIT/projects/19?pane=issue&itemId=56853714),
  [#82](https://github.com/CBIIT/ccdi-federation-api/pull/82)).
- Adds an endpoint to partition files by metadata elements and namespace ([link
  to
  card](https://github.com/orgs/CBIIT/projects/19?pane=issue&itemId=56853748),
  [#82](https://github.com/CBIIT/ccdi-federation-api/pull/82)).
- Adds support for reporting the metadata fields for file at
  `/metadata/file/fields` ([link to
  card](https://github.com/orgs/CBIIT/projects/19?pane=issue&itemId=56853672),
  [#82](https://github.com/CBIIT/ccdi-federation-api/pull/82)).
- Nine new metadata elements were added
  ([#89](https://github.com/CBIIT/ccdi-federation-api/pull/89) and
  [#91](https://github.com/CBIIT/ccdi-federation-api/pull/91)).
  - The library strategy for the sample (CDE 6273393 v1.00,
    [#48](https://github.com/CBIIT/ccdi-federation-api/discussions/48))
  - The preservation method for the sample's biospecimen (CDE 8028962 v2.00,
    [#64](https://github.com/CBIIT/ccdi-federation-api/discussions/64))
  - Disease at diagnosis (unaligned,
    [#52](https://github.com/CBIIT/ccdi-federation-api/discussions/#38)).
  - A short title for a study (CDE 11459812 v1.00,
    [#69](https://github.com/CBIIT/ccdi-federation-api/discussions/69)).
  - A funding id for a study (CDE 14528051 v1.00,
    [#66](https://github.com/CBIIT/ccdi-federation-api/discussions/66)).
  - An identifier for a study (CDE 12960571 v1.00,
    [#70](https://github.com/CBIIT/ccdi-federation-api/discussions/70)).
  - A name for a study (CDE 11459810 v1.00,
    [#68](https://github.com/CBIIT/ccdi-federation-api/discussions/68)).
  - Institution (CDE 12662779 v1.00,
    [#67](https://github.com/CBIIT/ccdi-federation-api/discussions/67)).
  - dbGaP phs Accession (CDE 11524544 v1.00,
    [#65](https://github.com/CBIIT/ccdi-federation-api/discussions/65)).

### Changed

- Revises the data model for files to more closely match subjects and samples by
  adding a `ccdi_server::responses::File` object
  ([#82](https://github.com/CBIIT/ccdi-federation-api/pull/82)).
- Ensures that all primary entities must explicitly exclude missing metadata
  objects for primary entities (subject, sample, and file) by assigning it a
  value of `null` rather than simply omitting the key
  ([#84](https://github.com/CBIIT/ccdi-federation-api/pull/84)).
- Revises the way missing or `null` results are displayed in group by endpoints.
  In particular, there is now a top-level `missing` key in those responses
  rather than using sentinel values to indicate missing data
  ([#83](https://github.com/CBIIT/ccdi-federation-api/pull/83)).
- Clarifies the language regarding the meaning of `null` within the API
  including guidance on when to return `null` and how to interpret `null`
  results (particularly when combined with enumerated values of CDEs that appear
  to overlap with this concept)
  ([#81](https://github.com/CBIIT/ccdi-federation-api/pull/81)).

### Fixed

- Adds filtering to the `/file` endpoint
  ([#88](https://github.com/CBIIT/ccdi-federation-api/pull/88)).

## [v0.7.0] — 03-25-2024

### Added

- Five new metadata elements were added for files ([#63](https://github.com/CBIIT/ccdi-federation-api/pull/63)).
  - The identifier for the file (CDE 11284037 v1.00, [#52](https://github.com/CBIIT/ccdi-federation-api/discussions/52))
  - The type of file (CDE 11416926 v1.00, [#53](https://github.com/CBIIT/ccdi-federation-api/discussions/53)).
  - The size of the file (CDE 11479876 v1.00, [#55](https://github.com/CBIIT/ccdi-federation-api/discussions/55)).
  - The md5 checksum of the file (CDE 11556150 v1.00, [#56](https://github.com/CBIIT/ccdi-federation-api/discussions/56)).
  - A description of the file (CDE 11280338 v1.00, [#54](https://github.com/CBIIT/ccdi-federation-api/discussions/54)).
- A `Namespace` now represents a top-level governance grouping of entities
  within the CCDI Federation API. See the new "Organizations", "Namespaces",
  and "Assigning Organizations and Namespaces" sections in the Swagger
  specification to learn about how your design might need to change to account
  for these new definitions
  ([#75](https://github.com/CBIIT/ccdi-federation-api/pull/75)).
- Introduces `Organization` as a supporting entity with the corresponding
  `/organization` and `/organization/{name}` endpoints
  ([#75](https://github.com/CBIIT/ccdi-federation-api/pull/75)).
- For all primary entities, an `Identifier` is now represented as both a
  `Namespace` identifier and a `Name` of the entity.
- Identifiers, when referenced from within metadata blocks (but not when specified as
  the top-level identifiers for subjects/samples/files) are now known as **referenced**
  identifiers. Their body type has changed to include `linked::Identifier`s and
  `unlinked::Identifier`s, which are wrapped in an `identifier::Referenced` enum
  ([#75](https://github.com/CBIIT/ccdi-federation-api/pull/75)).
- The `identifiers` metadata field for a `Subject` was changed from an _owned_
  metadata field to an _unowned_ metadata field
  ([#75](https://github.com/CBIIT/ccdi-federation-api/pull/75)).
- A new `identifiers` metadata field was added to `Sample` to support adding other
  known identifiers at the sample level
  ([#75](https://github.com/CBIIT/ccdi-federation-api/pull/75)).
- For both samples and subjects, there is a new `partition` query parameter that needs
  to be supported for the `/{entity}/by/{field}/count` endpoint
  ([#75](https://github.com/CBIIT/ccdi-federation-api/pull/75)).
    - The only current valid value for the `partition` query parameter at the moment is
      `namespace`, though this may be expanded in the future.

### Changed

- Pagination is now required by default for primary entity root endpoints
  (`/subject`, `/sample`, and `/file`)
  ([#59](https://github.com/CBIIT/ccdi-federation-api/pull/59)).
- HTTPS served on port 443 is now required
  ([#60](https://github.com/CBIIT/ccdi-federation-api/pull/60)).
- Updates the name of the API to be the "CCDI Data Federation API".

### Fixed

- The pagination via `Link` headers was fixed
  (Thanks @e-t-k! [#61](https://github.com/CBIIT/ccdi-federation-api/pull/61)).

## [v0.6.1] — 01-16-2024

### Changed

- Corrects two issues with the v0.6.0 release
  ([#51](https://github.com/CBIIT/ccdi-federation-api/pull/51)).
  - Corrects the description of the sample metadata `tumor_tissue_morphology` to be specified as a
    struct. Additionally, add the `icd_o_3` code as a named struct field so that it will be named in the wiki.
  - Adds the `age_at_vital_status` subject metadata to `get_field_descriptions()` so that it will be
    included in the wiki page generation.

## [v0.6.0] — 01-16-2024

### Added

- Adds the files and gateways API ([#31](https://github.com/CBIIT/ccdi-federation-api/pull/31)).
- Adds the `check` command for checking conformance with the API specification ([`c092b06`](https://github.com/CBIIT/ccdi-federation-api/commit/c092b064e4060471bdb6628b26a4099632c2089b)).
- Adds five new harmonized metadata elements ([#49](https://github.com/CBIIT/ccdi-federation-api/pull/49)).
  - Subjects now support vital status ([#42](https://github.com/CBIIT/ccdi-federation-api/discussions/42)) and age at vital status ([#45](https://github.com/CBIIT/ccdi-federation-api/discussions/45)).
  - Samples now support age at diagnosis ([#37](https://github.com/CBIIT/ccdi-federation-api/discussions/370)), age at collection ([#44](https://github.com/CBIIT/ccdi-federation-api/discussions/44)), and tumor tissue morphology ([#43](https://github.com/CBIIT/ccdi-federation-api/discussions/43)).

## [v0.5.0] — 11-25-2023

### Added

- Formalizes the `Namespace` entity and adds `/info` endpoint ([#27](https://github.com/CBIIT/ccdi-federation-api/pull/27)).
  - `/info`: Gets information about the server.
  - `/namespace`: Lists namespaces known by this server.
  - `/namespace/{name}`: Gets the namespace matching the provided name (if it exists).
- Adds a `InvalidRoute` error to give feedback in the response body when a route is not available ([#28](https://github.com/CBIIT/ccdi-federation-api/pull/28)).
- Adds an `UnshareableData` error to explain why data is not shareable ([#29](https://github.com/CBIIT/ccdi-federation-api/pull/29)).

## [v0.4.0] — 11-17-2023

### Added

- Adds filtering via query parameters for `/subject` and `/sample` ([#26](https://github.com/CBIIT/ccdi-federation-api/pull/26)).
- Adds a reference from samples back to subjects ([#25](https://github.com/CBIIT/ccdi-federation-api/pull/25)).
- Adds pagination to `/subject` and `/sample` ([#24](https://github.com/CBIIT/ccdi-federation-api/pull/24)).
- New server endpoints.
  - Adds Kids First Data Resource Center endpoint ([#30](https://github.com/CBIIT/ccdi-federation-api/pull/30)).

### Fixed

- Applies the `Refactory` -> `Refractory` typo present in the CDE 12217251 v1.00 ([#23](https://github.com/CBIIT/ccdi-federation-api/pull/23)).

## [v0.3.0] — 11-02-2023

### Changed

- Adds identifier with namespace to sample.
  - `/sample/{name}` now updated to `/sample/{namespace}/{name}`.

## [v0.2.0] — 11-02-2023

### Added

- New endpoints for `/sample`.
  - `/sample`: Gets the samples known by this server.
  - `/sample/{name}`: Gets the sample matching the provided name (if the sample exists).
  - `/sample/by/{field}/count`: Groups the samples by the specified metadata field and returns counts.
- Adds character restrictions to harmonized and unharmonized fields ([#19](https://github.com/CBIIT/ccdi-federation-api/pull/19)).
- Adds unharmonized metadata fields.
- Adds linting with [`spectral`](https://github.com/stoplightio/spectral).

### Changed

- Removes `null`-based metadata fields.
- Supports full set of subject fields in group by count.
- Unharmonized keys can be any json string.
- Versions are now workspace-wide and in sync with API version

### Fixed

- Fixed swapping of server descriptions ([#18](https://github.com/CBIIT/ccdi-federation-api/pull/18)).

## [v0.1.0] — 10-15-2023

### Added

- New endpoints for subjects.
  - `/subject`: Gets the subjects known by this server.
  - `/subject/{namespace}/{name}`: Gets the subject matching the provided id (if
    it exists).
  - `/subject/by/{field}/count`: Groups the subjects by the specified metadata field and returns counts.
- New endpoints for metadata.
  - `/metadata/fields/subject`: Gets the metadata fields for subjects that are supported by this server.
- New server endpoints.
  - Adds St. Jude Children's Research Hospital endpoint (initial commit).
  - Adds UCSC Treehouse endpoint ([#6](https://github.com/CBIIT/ccdi-federation-api/pull/6)).
  - Adds Pediatric Cancer Data Commons (PCDC) endpoint ([#10](https://github.com/CBIIT/ccdi-federation-api/pull/10)).
- Rust tooling was added to the `packages` directory ([#14](https://github.com/CBIIT/ccdi-federation-api/pull/14)).

[Unreleased]: https://github.com/cbiit/ccdi-federation-api/compare/v1.0.0-rc.1...HEAD
[v1.0.0-rc.1]: https://github.com/cbiit/ccdi-federation-api/compare/v0.7.0...v1.0.0-rc.1
[v0.7.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.6.1...v0.7.0
[v0.6.1]: https://github.com/cbiit/ccdi-federation-api/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/cbiit/ccdi-federation-api/releases/tag/v0.0.1
