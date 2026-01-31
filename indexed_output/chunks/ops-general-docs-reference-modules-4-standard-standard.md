---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#4-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 521
summary: a module is required at two different versions by transitive dependencies, the. higher version will be used
---

a module is required at two different versions by transitive dependencies, the
higher version will be used. However, if the two versions are incompatible,
neither version will satisfy all clients. Since incompatible versions must have
different major version numbers, they must also have different module paths due
to major version suffixes. This resolves the conflict: modules with distinct
suffixes are treated as separate modules, and their packages—even packages in
same subdirectory relative to their module roots—are distinct.

MAJOR VERSION DEFAULTS

When a package import path does not contain a major version,
the module.cue file is consulted to determine which major
version of the module to use. In a canonical module.cue file,
all imports without major versions will have an explicit default: true
present in the corresponding dependency entry, but cue mod tidy
will add those if not present and there is no ambiguity in the build list.

That is, given only a single major version of a module in the build list,
the major version need not be specified in any of the package imports.

RESOLVING A PACKAGE TO A MODULE

When CUE loads a package using a package
path [/docs/reference/modules/#glos-package-path], it needs to determine which module provides the
package.

It starts by searching the build list [/docs/reference/modules/#glos-build-list] for
modules with paths that are prefixes of the package path. For example, if the
package foo.example/a/b is imported, and the module foo.example/a is in the
build list, CUE will check whether foo.example/a contains the
package, in the directory b. At least one file with the .cue extension must
be present in a directory for it to be considered a package. Build
constraints [/docs/reference/modules/#glos-build-constraint] are not applied for this
purpose. If exactly one module in the build list provides the package, that
module is used. If no modules provide the package or if two or more modules
provide the package, CUE reports an error. The cue mod tidy command
will attempt to find new modules providing missing
