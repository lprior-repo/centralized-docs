---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#17-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: the major version need not be specified in any of the package imports. RESOLVING A PACKAGE TO A MODULE
---

the major version need not be specified in any of the package imports.

RESOLVING A PACKAGE TO A MODULE

When CUE loads a package using a package
path [/docs/reference/modules/#glos-package-path], it needs to determine which module provides the
package.

It starts by searching the build list [/docs/reference/modules/#glos-build-list] for
modules with paths that are prefixes of the package path. For example, if the
package foo.example/a/b is imported, and the module foo.example/a is in the
build list, CUE will check whether foo.example/a contains the
