# Assigning Namespaces and Organizations

When assigning namespaces within a source server, one should consider making a
namespace entity for each grouping of primary entities that are governed under a
common model.

Here are some common situations followed by instructive examples of how you
partition primary entities to a set of namespaces under that situation:

- If all primary entities within your source server are governed by a singular
  governing body (say, a single data access committee), then you may only need
  one namespace for all primaries entities within your server.
- If you have multiple data access committees governing different groupings of
  primary entities from the same institution, you should create multiple
  namespaces that are backed by a common organization.
- If your server serves data from various governing bodies spread across
  multiple, independent organizations, you should create multiple namespaces
  backed by multiple organizations.
