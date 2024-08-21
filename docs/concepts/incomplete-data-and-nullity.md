# Incomplete Data and Nullity

_Note: strings are explicitly enclosed in double quotes (`"`) to distinguish
their string values from `null` in this section._

Metadata elements within this specification are harmonized to [Common Data
Elements](https://datascience.cancer.gov/resources/metadata) (CDEs) cataloged
at the [Cancer Data Standards Registry and
Repository](https://cadsr.cancer.gov) (caDSR). These CDEs are drafted by a
diverse set of stakeholders for a variety of purposes and, as such, they lack a
cross-CDE mechanism for the expressing the spectrum of incompleteness or
confidence that accompany expressing data elements.

To illustrate this concept, consider that the CDE selected to represent [vital
status](https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#vital_status)
for subjects is relatively comprehensive in its permissible values (`"Not
reported"`, `"Unknown"`, `"Unspecified"`), whereas the CDE selected to
represent [tumor
morphology](https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#tumor_tissue_morphology)
for samples has no such equivalent facilities to express an unknown or
incomplete value. Because uniformity amongst reporting metadata elements is
desirable to simplify parsing, a mechanism for explicit omission of information
is needed as a fallback across all data elements.

`null` values, as present in this specification, represent a lack of assertion
(or, equivalently, a "decline to comment") by the server. This value should not
be construed to communicate _any_ content about the field it represents—either
in affirmation or dissent. Instead, they should be interpreted simply as an
explicit omission of data that can be updated at any time if new information is
made available to the source server.

This is in stark contrast to explicit values of `"Unknown"` or `"Not
Reported"`, which often have explicit meaning in the context of the scope of
the CDE. Sometimes, these permissible values (particularly the `"Not Reported"`
value) venture into overlapping territory with the aforementioned definition of
`null` values. This introduces potential ambiguity when deciding which value to
use when representing nullity in a server's responses (or, conversely, how to
interpret these different representations of nullity when retrieving data from
multiple servers).

Use the following guidelines when deciding when to use `"Unknown"`/`"Not
Reported"`/`"Unavailable"`, when to use `null`, and how to interpret the values
in your API calls.

- First (and most importantly), make sure you carefully read and understand the
  context of the CDE by visiting the [caDSR](https://cadsr.cancer.gov) (or the
  [wiki pages](https://github.com/CBIIT/ccdi-federation-api/wiki)) and
  reviewing the CDE's "Definition." For example, a participant self-reporting a
  value of `"Unknown"` to a demographics question is _not_ the same as an
  investigator indicating `"Unknown"` for a value that is simply missing from
  their records. **Ultimately, all CDEs and their respective permissible values
  should be interpreted strictly in that context**.
- If the CDE provides a permissible value that covers the particular flavor of
  incomplete or unknown information in your case, **use the explicit
  permissible value assigned by the CDE**.
- If there is no permissible value that covers the particular flavor of
  incomplete or unknown information in your case, **use the `null` value to
  indicate that the server is explicitly declining to comment**. The authors
  recommend that you note why data is missing in the associated
  `/metadata/*/fields` entry to increase the likelihood that users interpret
  your values correctly. Additionally, please be sure to check the
  specification to ensure that a value of `null` is actually allowed!
- Interpreting equivalence of values such as `"Unknown"`, `"Not Reported"`,
  `"Unavailable"`, and `null` is a context-specific decision based on (a) the
  definition of the CDE and (b) the context in which you're applying the
  information. If the appropriate course of action for your situation is not
  immediately apparent, we ask that you file a [an API question on the GitHub
  Discussions
  page](https://github.com/CBIIT/ccdi-federation-api/discussions/categories/api-questions)
  so the community can come together to help answer your question.
