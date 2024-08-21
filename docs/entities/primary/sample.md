# Samples

**Samples** represent a biological entity captured at a single point in time
upon which an experiment has been performed. Classification as a sample does not
imply resection: a sample _may_ have been resected (in the case of, say, omics
profiling), or it _may not_ be resected (in the case of imaging data).

The current design combines the concepts of the physical, biological specimen,
the experiment run on that specimen, the time point at which the experiment was
done into one entry. This approach simplifies the design for `v1` of the API but
carries an important caveat: **samples do not purport to confer information
regarding unique cases or biospecimen**, and you should be careful not to
interpret them as such. As we work towards modeling biospecimens and diagnoses
more formally in future versions of the API, we expect the flexibility and
conciseness with which you can ask a variety of different questions to improve.

::: warning

The caveat mentioned above means that,

- When a single case has been profiled by whole-genome, whole-exome, and
  RNA-Seq, the server will return three sample entries—not one. Importantly
  though, the identifier for the sample should be made unique between the three
  entries when also considering the experiment in the identifier.
- When the same tumor-banked biospecimen is sequenced by two independent
  researchers working on different research projects and data from both projects
  is included in the source server, that server will return two sample entries
  in that instance—not one. Importantly though, the identifier for the sample
  should be made unique between the two entries when also considering the
  namespace in the identifier (different projects should have different
  namespaces).

This also means that simply counting the number of _cases_ of a particular
diagnosis is not as simple as using the `GET /sample/by/diagnosis/count`
endpoint. When counting samples within a particular source server, you should
consider deduplicating samples across namespaces (if you don't want the same
biological entity sequenced across multiple projects to count multiple times)
and/or deduplicating by experiment (if you don't want multiple experiments being
performed against the same biological entity to be counted multiple times).
That being said, comprehensive deduplication of cases across the ecosystem is
not currently supported for a variety of other reasons as well, including the
fact that no facility exists to deduplicate samples across source servers.

:::
