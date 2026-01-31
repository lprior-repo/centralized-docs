---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary:    that does not affect the module’s public interface, such as a bug fix or.    change to the documentation
---

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
