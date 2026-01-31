---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
summary: that map to different CUE packages within the same module. proto files import each other, and other centralized schema
---

that map to different CUE packages within the same module.
If several .proto files import each other, and other centralized schema
definitions, then things can get hairy!
In these situations, CUE’s Go API and the cue command have you covered.

Both cue import [/docs/reference/command/cue-help-import/] and the
encoding/protobuf [https://pkg.go.dev/cuelang.org/go/encoding/protobuf]
package can be configured to handle custom import paths but, by default, when
they encounter …

 * .proto files that have a go_package directive: CUE uses this path
 * files that map to a package within the CUE module: CUE uses the package’s directory
 * any other import path: CUE maps to a location in the cue.mod/pkg directory.

EXPERIMENTAL APIS

CUE initially publishes APIs and packages marked as “experimental”, in order to
gather feedback on their use and structure before comitting the project to
their long-term support.
CUE’s Protobuf APIs include two experimental packages:
encoding/protobuf/textproto [https://pkg.go.dev/cuelang.org/go/encoding/protobuf/textproto]
and
encoding/protobuf/jsonpb [https://pkg.go.dev/cuelang.org/go/encoding/protobuf/jsonpb].

textproto converts
text Protobuf message files [https://protobuf.dev/reference/protobuf/textformat-spec/]
to and from CUE, and jsonpb rewrites a CUE expression based on the Protobuf
interpretation of JSON.

Your feedback on their utility and structure is invaluable - please join the
CUE community [/community/] on Slack and GitHub, and let us
know how you’re using these APIs!

PROTOBUF MAPPINGS

The mappings between Protobuf and CUE types are outlined in the encoding/protobuf
package documentation [https://pkg.go.dev/cuelang.org/go/encoding/protobuf#hdr-Type_Mappings].

FUTURE PLANS

CUE’s support for Protobuf is only going to expand, with plans including the
conversion of CUE definitions to binary Protobuf definitions, and for
bidirectional conversion of binary and JSON Protobuf messages to and from
CUE.

RELATED CONTENT

 * Reference: cue help import [/docs/reference/command/cue-help-import/]
