---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#18-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: package, in the directory b.  At least one file with the 
---

package, in the directory b. At least one file with the .cue extension must
be present in a directory for it to be considered a package. Build
constraints [/docs/reference/modules/#glos-build-constraint] are not applied for this
purpose. If exactly one module in the build list provides the package, that
module is used. If no modules provide the package or if two or more modules
provide the package, CUE reports an error. The cue mod tidy command
will attempt to find new modules providing missing
packages and to update cue.mod/module.cue accordingly.
