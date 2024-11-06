**PR Close Date:** [ENTER DATE HERE]

_Describe the problem or feature in addition to a link to the issues._

Before submitting this PR, please make sure:

- [ ] You have added a few sentences describing the PR here.
- [ ] You have added yourself or the appropriate individual as the assignee.
- [ ] You have added the relevant groups/individuals to the reviewers.
- [ ] Your commit messages conform to the [Conventional
      Commits](https://www.conventionalcommits.org/en/v1.0.0/) standard.
- [ ] You have updated the README or other documentation to account for these
      changes (when appropriate).
- [ ] You have added a line describing the change in the `CHANGELOG.md` under
      `[Unreleased]`.

<!--

If you are adding new metadata elements, please uncomment this section and
complete the checklist as well:

- [ ] I have added my field definition to the appropriate
  `get_field_descriptions()` method. For example, if you add a field to
  subjects, you should include it in the `get_field_descriptions()` method at
  `packages/ccdi_models/src/metadata/field/description/harmonized/subject.rs`.
- [ ] I have confirmed that my field shows up in the relevant
  `/metadata/fields/<entity>` endpoint. For example. if you add a field to
  subjects, it should show up in the fields listed in the output of the
  `/metadata/fields/subject` endpoint.
- [ ] I have confirmed that my field filters correctly when filtered from the
  root endpoint (`/subject`, `/sample`, etc). For example, if you add the
  `anatomical_site` field to the sample endpoint, make sure that visiting
  `http://localhost:8000/sample?anatomical_site=foobar` works.
- [ ] I have confirmed that my field shows up in the relevant wiki generation
  command. For example. if you add a field to subjects, it should show up in the
  `cargo run --release wiki subject` output.

-->
