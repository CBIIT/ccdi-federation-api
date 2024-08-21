<p align="center">
  <h1 align="center">
  CCDI Data Federation API Specification
  </h1>
</p>

  <p align="center">
    <a href="https://github.com/CBIIT/ccdi-federation-api/actions/workflows/specification.yml" target="_blank">
      <img alt="CI: Specification Status" src="https://github.com/CBIIT/ccdi-federation-api/actions/workflows/specification.yml/badge.svg" />
    </a>
    <a href="https://github.com/CBIIT/ccdi-federation-api/actions/workflows/specification-tool.yml" target="_blank">
      <img alt="CI: Specification Status" src="https://github.com/CBIIT/ccdi-federation-api/actions/workflows/specification-tool.yml/badge.svg" />
    </a>
    <a href="https://github.com/CBIIT/ccdi-federation-api/blob/main/LICENSE-APACHE" target="_blank">
      <img alt="License: Apache 2.0" src="https://img.shields.io/badge/license-Apache 2.0-blue.svg" />
    </a>
    <a href="https://github.com/CBIIT/ccdi-federation-api/blob/main/LICENSE-MIT" target="_blank">
      <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-blue.svg" />
    </a>
  </p>

<p align="center">
    An API for querying federated pediatric cancer data from the broader community.
    <br />
    <a href="https://cbiit.github.io/ccdi-federation-api/"><strong>Read the docs »</strong></a>
    <br/>
    <br />
    <a href="https://github.com/cbiit/ccdi-federation-api/issues/new?assignees=&labels=&template=feature_request.md&title=Descriptive%20Title&labels=enhancement">Request Feature</a>
    ·
    <a href="https://github.com/cbiit/ccdi-federation-api/issues/new?assignees=&labels=&template=bug_report.md&title=Descriptive%20Title&labels=bug">Report A Bug</a>
    <br />
  </p>
</p>

> [!IMPORTANT]
> If you're looking to learn more about the service that aggregates federation
> responses into a single response (i.e., the CCDI Data Federation Resource),
> [click here](https://github.com/CBIIT/ccdi-federation-api-aggregation).

## Developing

The specification is generated using the [Rust] packages contained with
`packages` directory. In particular, [`utoipa`] is used to autogenerate the
OpenAPI 3.0 specification. An [Actix Web] server is provided that (a) provides
the foundation for `utoipa` to generate the API documentation and (b) provides
an example server using fake data. Please refer to the [Learn Rust] guide to
learn how to develop using Rust.

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check
[issues page](https://github.com/CBIIT/ccdi-federation-api/issues).

### Development Process

-   Changes are first discussed in the
    [discussions](https://github.com/cbiit/ccdi-federation-api/discussions)
    section of the repo. The purpose of these discussions is to describe your
    idea(s), receive feedback from other implementors, and ultimately gain support
    within the community for these features.
-   Once a set of changes has been approved via the discussions mechanism, work
    can commence on a [pull
    request](https://github.com/cbiit/ccdi-federation-api/discussions)
    implementing these changes.

### Repository Details

-   This repository uses the [Conventional
    Commit](https://www.conventionalcommits.org/en/v1.0.0/) style for commit
    messages. Please make sure all commits conform to this style.
-   This tooling that produces the API specification are versioned using the
    latest version of [Semantic Versioning](https://semver.org/). The API itself
    is versioned according to the tooling that produced the specification (i.e.,
    v1.0.0 of the tooling produces v1.0.0 of the specification).
-   All changes will either be squashed and merged or rebased off of the `main`
    branch—no merge commits are allowed in this repository.

## License

This project is licensed as either [Apache 2.0][license-apache] or
[MIT][license-mit] at your discretion.

Copyright © 2023-Present Childhood Cancer Data Initiative Federation

[Actix Web]: https://actix.rs/
[Learn Rust]: https://www.rust-lang.org/learn
[Rust]: https://www.rust-lang.org/
[`utoipa`]: https://github.com/juhaku/utoipa
[license-apache]: https://github.com/CBIIT/ccdi-federation-api/blob/main/LICENSE-APACHE
[license-mit]: https://github.com/CBIIT/ccdi-federation-api/blob/main/LICENSE-MIT
