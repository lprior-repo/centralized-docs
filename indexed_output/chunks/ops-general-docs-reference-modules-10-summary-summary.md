---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: major, minor, and patch versions, from left to right) separated by dots. patch version may be followed by an optional pre-release string starting with a
---

major, minor, and patch versions, from left to right) separated by dots. The
patch version may be followed by an optional pre-release string starting with a
hyphen.

Each part of a version indicates whether the version is stable and whether it is
compatible with previous versions.

 * The major version must be incremented and the minor
   and patch versions must be set to zero after a backwards incompatible change
   is made to the module’s public interface or documented functionality, for
   example, after a package is removed.
