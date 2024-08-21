# Primary Entities

Primary entities represent information that this API specification was created
to share as a top-level goal. Primary entities have a common API surface and,
generally, will work relatively similar to one another within the
specification. All primary entities are referred to by their _identifier_,
which is the combination of (a) a namespace identifier pointing to the
namespace that owns this entity along with (b) a name for the entity.

The following entities are considered primary entities within the API
specification.

- [Subjects](./primary/subject.md)
- [Samples](./primary/sample.md)
- [Files](./primary/file.md)

Beyond merely existing in a common level of prominence within the API, primary
entities have a hierarchical structure following these rules.

- Subjects are the highest-level primary entity within the API specification.
- Samples **must** be associated with one and only one subject.
- Files **must** be associated with one or more samples.
