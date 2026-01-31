---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#49-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: module [/docs/reference/modules/#glos-main-module].  Each vertex in the graph is a module; each edge is a
---

module [/docs/reference/modules/#glos-main-module]. Each vertex in the graph is a module; each edge is a
version from an entry in the deps field in a cue.mod/module.cue file.

module path: A path that identifies a module and acts as a prefix for
package import paths within the module. For example, "cuelang.org/x/foo".

module root directory: The directory that contains the cue.mod/module.cue file that
defines a module.

package: A collection of source files, usually in the
same directory, that are evaluated together. See the Packages
