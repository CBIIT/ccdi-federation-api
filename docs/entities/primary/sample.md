# Samples

**Samples** represent a biological entity measured at a single point in
time upon which an experiment has been performed. The current design is akin to
an _experimental_ sample and each entry encompasses the concepts of the physical,
biological specimen, the experiment run on that specimen, and the time point at
which the experiment was done. This approach simplifies the design for v1 of the
API but carries an important caveat: samples are not necessarily unique cases or
biospecimens, and you should be careful not to interpret them as such. As we
work towards modeling biospecimens and diagnoses more formally in future
versions of the API, we expect to improve the flexibility and conciseness with
which you can ask a variety of different questions.

::: warning

The caveat mentioned above means that,

- When a single case has been profiled by whole-genome, whole-exome, and
  RNA-Seq, the server is expected to return three sample entries—not one.
  Importantly though, the identifier for each sample should be made unique via
  the identifier. Further, the
  [`library_strategy`](https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#library_strategy)
  can be used to understand which strategy was used to sequence each sample.
- When the same tumor-banked biospecimen is sequenced by two independent
  researchers working on different research projects and data from both projects
  is included in the source server, that server is expected to return two sample
  entries in that instance—not one. Importantly though, the identifier for the
  sample should be made unique between the two entries when also considering the
  namespace in the identifier (different projects should have different
  namespaces).

This also means that simply counting the number of _cases_ of a particular
diagnosis is not as simple as using the `GET /sample/by/diagnosis/count`
endpoint. When counting samples within a particular source server, you should
consider deduplicating samples across namespaces (if you don't want the same
biological entity sequenced across multiple projects to be counted multiple
times) and/or deduplicating by experiment (if you don't want multiple
experiments being performed against the same biological entity to be counted
multiple times). Even in doing so, a comprehensive deduplication of cases across
the ecosystem is not currently possible for a variety of other reasons,
including the fact that no facility exists to deduplicate samples across source
servers.

:::


For more information, take a look at the relevant
[API
calls](https://cbiit.github.io/ccdi-federation-api/specification.html#tag/sample)
and
[models](https://cbiit.github.io/ccdi-federation-api/specification.html#model/modelssample).
