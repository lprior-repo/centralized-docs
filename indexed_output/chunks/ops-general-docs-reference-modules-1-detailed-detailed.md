---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1033
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
   that does not affect the module’s public interface, such as a bug fix or
   change to the documentation.
 * The pre-release suffix indicates a version is a pre-release. Pre-release versions sort before
   the corresponding release versions. For example, v1.2.3-pre comes before
   v1.2.3.

A version is considered unstable if its major version is 0 or it has a
pre-release suffix. Unstable versions are not subject to compatibility
requirements. For example, v0.2.0 may not be compatible with v0.1.0, and
v1.5.0-beta may not be compatible with v1.5.0.

MAJOR VERSION SUFFIXES

Module paths must have a major version
suffix like @v2 that matches the major version. For example, if a module
has the path foo.example/mod@v1 at v1.0.0, it must have the path
foo.example/mod@v2 at version v2.0.0.

Major version suffixes implement the import compatibility
rule [https://research.swtch.com/vgo-import]:

> If an old package and a new package have the same import path,
> the new package must be backwards compatible with the old package.

By definition, packages in a new major version of a module are not backwards
compatible with the corresponding packages in the previous major version.
Consequently each new major version of a package needs a new import path.
This is accomplished by adding a major version suffix to the module path.
The import path for a package also includes the major version suffix,
providing a distinct import path for each incompatible version.

Unlike in Go [https://go.dev/ref/mod#major-version-suffixes],
major version suffixes are always required in module paths . The burden
of changing import paths in packages is eased by allowing the
major version suffix to be omitted and inferred from the module.cue
file. See major version defaults [/docs/reference/modules/#major-version-defaults] for details.

Major version suffixes let multiple major versions of a module coexist in the
same build. This may be necessary due to a diamond dependency
problem [https://research.swtch.com/vgo-import#dependency_story]. Ordinarily, if
