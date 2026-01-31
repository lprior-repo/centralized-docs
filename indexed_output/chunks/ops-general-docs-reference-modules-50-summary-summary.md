---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#50-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 144
summary: section [https://cuelang. org/docs/references/spec/#modules-instances-and-packages]
---

section [https://cuelang.org/docs/references/spec/#modules-instances-and-packages]
in the CUE Language Specification.

package path: The path that uniquely identifies a package. A package path is
a module path [/docs/reference/modules/#glos-module-path] joined with a subdirectory within the module.
For example "cuelang.org/x/foo/html" is the package path for the package in the
module "cuelang.org/x/foo" in the "html" subdirectory. Synonym of
import path [/docs/reference/modules/#glos-import-path].

patch version: The third number in a semantic version (3 in v1.2.3). In
