---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 147
summary: definitions, then things can get hairy!. In these situations, CUE’s Go API and the cue command have you covered
---

definitions, then things can get hairy!
In these situations, CUE’s Go API and the cue command have you covered.

Both cue import [/docs/reference/command/cue-help-import/] and the
encoding/protobuf [https://pkg.go.dev/cuelang.org/go/encoding/protobuf]
package can be configured to handle custom import paths but, by default, when
they encounter …

 * .proto files that have a go_package directive: CUE uses this path
 * files that map to a package within the CUE module: CUE uses the package’s directory
 * any other import path: CUE maps to a location in the cue.mod/pkg directory.
