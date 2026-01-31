---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: to and from CUE, and jsonpb rewrites a CUE expression based on the Protobuf. interpretation of JSON
---

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
