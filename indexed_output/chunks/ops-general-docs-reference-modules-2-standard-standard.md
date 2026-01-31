---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
summary: to the restrictions. documented there [https://github
---

to the restrictions
documented there [https://github.com/opencontainers/distribution-spec/blob/main/spec.md#pulling-manifests].
To summarize:

 * The path must consist of one or more path elements separated by slashes
   (/, U+002F). It must not begin or end with a slash.
 * No characters are allowed in the path except lower case ASCII letters,
   ASCII digits, and limited ASCII punctuation (-, _, .).
 * The first character of each path element is a letter or a digit.
 * No more than one period (.) is allowed in sequence.
 * No more than two underscores (_) are allowed in sequence.

In addition, the first path element must contain at least one period character (.).

No restriction is directly enforced on the length of module names, but as registries
can refuse module paths over 128 characters, and it’s possible to specify
an arbitrary storage prefix, long module paths may fail.

VERSIONS

A version identifies an immutable snapshot of a module, which may be
either a release or a
pre-release (with a pre-release suffix). Each version starts with the letter
v, followed by a semantic version. See
Semantic Versioning 2.0.0 [https://semver.org/spec/v2.0.0.html] for details on how versions are
formatted, interpreted, and compared.

To summarize, a semantic version consists of three non-negative integers (the
major, minor, and patch versions, from left to right) separated by dots. The
patch version may be followed by an optional pre-release string starting with a
hyphen.

Each part of a version indicates whether the version is stable and whether it is
compatible with previous versions.

 * The major version must be incremented and the minor
   and patch versions must be set to zero after a backwards incompatible change
   is made to the module’s public interface or documented functionality, for
   example, after a package is removed.
 * The minor version must be incremented and the patch
   version set to zero after a backwards compatible change, for example, after a
   new function is added.
 * The patch version must be incremented after a change
