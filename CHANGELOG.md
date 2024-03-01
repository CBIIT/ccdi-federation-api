# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Updates the name of the API to be the "CCDI Data Federation API".

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

[Unreleased]: https://github.com/cbiit/ccdi-federation-api/compare/v0.6.1...HEAD
[v0.6.1]: https://github.com/cbiit/ccdi-federation-api/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/cbiit/ccdi-federation-api/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/cbiit/ccdi-federation-api/releases/tag/v0.0.1
