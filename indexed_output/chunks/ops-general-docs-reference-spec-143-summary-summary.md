---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#143-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: package math. MODULES AND INSTANCES
---

name.


Copy code
Copied!

package math

MODULES AND INSTANCES

A module defines a tree of directories, rooted at the module root.

All source files within a module with the same package name belong to the same
package.

A module may define multiple packages.

An instance of a package is any subset of files belonging
to the same package.

It is interpreted as the concatenation of these files.

An implementation may impose conventions on the layout of package files
to determine which files of a package belongs to an instance.
