# Changelog

## 0.5.0 — 11-25-2023

### Added

* Formalizes the `Namespace` entity and adds `/info` endpoint ([#27](https://github.com/CBIIT/ccdi-federation-api/pull/27)).
  * `/info`: Gets information about the server.
  * `/namespace`: Lists namespaces known by this server.
  * `/namespace/{name}`: Gets the namespace matching the provided name (if it exists).
* Adds a `InvalidRoute` error to give feedback in the response body when a route is not available ([#28](https://github.com/CBIIT/ccdi-federation-api/pull/28)).
* Adds an `UnshareableData` error to explain why data is not shareable ([#29](https://github.com/CBIIT/ccdi-federation-api/pull/29)).

## 0.4.0 — 11-02-2023

### Added

* Adds filtering via query parameters for `/subject` and `/sample` ([#26](https://github.com/CBIIT/ccdi-federation-api/pull/26)).
* Adds a reference from samples back to subjects ([#25](https://github.com/CBIIT/ccdi-federation-api/pull/25)).
* Adds pagination to `/subject` and `/sample` ([#24](https://github.com/CBIIT/ccdi-federation-api/pull/24)).
* New server endpoints.
    * Adds Kids First Data Resource Center endpoint ([#30](https://github.com/CBIIT/ccdi-federation-api/pull/30)).

### Fixes

* Applies the `Refactory` -> `Refractory` typo present in the CDE 12217251 v1.00 ([#23](https://github.com/CBIIT/ccdi-federation-api/pull/23)).

## 0.3.0 — 11-02-2023

### Revise

* Adds identifier with namespace to sample.
    * `/sample/{name}` now updated to `/sample/{namespace}/{name}`.

## 0.2.0 — 11-02-2023

### Added

* New endpoints for `/sample`.
    * `/sample`: Gets the samples known by this server.
    * `/sample/{name}`: Gets the sample matching the provided name (if the sample exists).
    * `/sample/by/{field}/count`: Groups the samples by the specified metadata field and returns counts.
* Adds character restrictions to harmonized and unharmonized fields ([#19](https://github.com/CBIIT/ccdi-federation-api/pull/19)).
* Adds unharmonized metadata fields.
* Adds linting with [`spectral`](https://github.com/stoplightio/spectral).

### Revised

* Removes `null`-based metadata fields.
* Supports full set of subject fields in group by count.
* Unharmonized keys can be any json string.

### Fixes

* Fixed swapping of server descriptions ([#18](https://github.com/CBIIT/ccdi-federation-api/pull/18)).

### Important Chores

* Versions are now workspace-wide and in sync with API version

## 0.1.0 — 10-15-2023

### Added

* New endpoints for subjects.
  * `/subject`: Gets the subjects known by this server.
  * `/subject/{namespace}/{name}`: Gets the subject matching the provided id (if
    it exists).
  * `/subject/by/{field}/count`: Groups the subjects by the specified metadata field and returns counts.
* New endpoints for metadata.
  * `/metadata/fields/subject`: Gets the metadata fields for subjects that are supported by this server.
* New server endpoints.
    * Adds St. Jude Children's Research Hospital endpoint (initial commit).
    * Adds UCSC Treehouse endpoint ([#6](https://github.com/CBIIT/ccdi-federation-api/pull/6)).
    * Adds Pediatric Cancer Data Commons (PCDC) endpoint ([#10](https://github.com/CBIIT/ccdi-federation-api/pull/10)).
* Rust tooling was added to the `packages` directory ([#14](https://github.com/CBIIT/ccdi-federation-api/pull/14)).