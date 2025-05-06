### Table of Contents
- [Local Setup](#local-setup)
  - [Inital setup](#initial-setup)
  - [Running locally](#running-the-server-locally)
- [Development](#development)
  - [Steps before reviews](#setting-up-changes-for-review)
  - [Downloading Uberon ontology](#downloading-uberon-ontology)
- [Using the app](#using-the-app)
  - [Validating endpoints](#validating-endpoints-against-the-reference-implementation)

<br><br>

# Local Setup

## Initial setup

- [Install Rust](https://www.rust-lang.org/tools/install). This will get you the tools rustc (compiler), rustup (updater), and cargo (package management).
- Switch to the "nightly build":
    - `rustup toolchain install nightly` to install
    - `rustup default nightly` to use nightly build
- Clone the repo: `git clone https://github.com/CBIIT/ccdi-federation-api.git`
- Go into the rust `crates` directory: `cd ccdi-federation-api/crates`. (This is the working directory for all following commands)
- Check out your desired branch, e.g.: `git checkout feat/restrict-charsets`

The repo also uses [`spectral`](https://docs.stoplight.io/docs/spectral/) which is a tool for linting the OpenAPI specification YAML file.
To install it, you need to [install node](https://nodejs.org/en/download) first and install `spectral` on your machine by running `npm install -g @stoplight/spectral-cli`.

## Running the server locally

- To get the reference server running locally, run `cargo run --bin ccdi-spec serve`.
This will build all the dependencies (takes a minute the first time and is quicker thereafter) and then start a server on http://localhost:8000/.

- To visit the swagger spec in the browser, navigate to http://localhost:8000/swagger-ui/ (the trailing slash is required!)

- You can also visit an API implementation with example data by going to the appropriate endpoints, e.g. http://localhost:8000/sample/by/tumor_classification/count.

<br>

# Development 

Some tips for development:
- It is recommended to install and use the `rust-analyzer` plugin for your IDE while developing.
- To view changes made on the server, you will need to interrupt the server (e.g. Ctrl-C) and start it up again with `cargo run serve`, as well as refresh the page in the browser.
- Don't edit the `swagger.yml` file directly. Instead, regenerate the file once your code changes have been made.
While developing, you can do so by running `cargo run --bin ccdi-spec generate > ../swagger.yml`. This won't include the anatomical sites in the swagger document which is required
for the final swagger document generation.
- The default swagger compilation omits many anatomical sites. This is probably what you want while testing.
When doing a final commit, include the anatomical sites by running the following, which takes several minutes: `cargo run --bin ccdi-spec --features all-anatomical-site generate > ../swagger.yml`.

## Setting up changes for review
When your code changes are ready for review, run the following before making a PR and fix any issues (these checks are also performed as GitHub actions on the PR):

```
# Check that all linting and formatting passes.
cd crates && cargo fmt && cargo test --all-features && cargo doc && cargo clippy -- -D warnings; cd ..

# Generate a new version of the spec, make sure it lints okay, then copy to
# pasteboard (on Mac) so that you can view it in editor.swagger.io.
#
# Note: you can also run the server and visit the specification at 
# http://localhost:8000/swagger-ui/.
cd crates && cargo run --bin ccdi-spec --features all-anatomical-site generate -o ../swagger.yml -f && cd .. && spectral lint swagger.yml && cat swagger.yml | pbcopy
```

> [!IMPORTANT]
> <b> When creating a PR, please read through each point in the template, as there are additional required steps for any change (like updating the CHANGELOG.md file), and see if they are applicable to your changes.
> Adding new metadata elements have a longer checklist which is part of the PR template. </b>

### Downloading Uberon ontology

To download the Uberon ontology and compile the relevant rust classes, use `cargo run --release --bin ccdi-curate uberon -vv`.

<br>

# Using the app

### Validating endpoints against the reference implementation

You can validate your endpoints vs the reference implementation to confirm that they conform.

The command to do this is check: `cargo run --bin ccdi-spec check <URL> <RESPONSE_TYPE>`.

<u>Examples:</u>

`cargo run --bin ccdi-spec check "https://ccdi.treehouse.gi.ucsc.edu/api/v1/info" Information`

`cargo run --bin ccdi-spec check "https://ccdi.treehouse.gi.ucsc.edu/api/v1/sample?page=2&diagnosis=9380/3 : Glioma, malignant" Samples`

If the bottom line is `Success!`, the response was conformant (this may not catch every error).
Otherwise, you will get an error. For example:

    error: data did not match any variant of untagged enum Description at line 1 column 329.
    error: missing field counts at line 10 column 1

This gives you a hint of where in the JSON your endpoint response diverged from what the reference implementation was expecting.

Possible `RESPONSE_TYPE`s, also listed when you call `cargo run --bin ccdi-spec check --help`:

Samples, Sample, SamplesByCount, Subjects, Subject, SubjectsByCount, Files, Namespaces, Namespace, Organizations, Organization, Summary, Information, FieldDescriptions, Errors

