---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1032
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
 * Reference: cue help filetypes [/docs/reference/command/cue-help-filetypes/]
 * Go API: encoding/protobuf [https://pkg.go.dev/cuelang.org/go/encoding/protobuf]
 * Go API: encoding/protobuf/textproto [https://pkg.go.dev/cuelang.org/go/encoding/protobuf/textproto]
 * Go API: encoding/protobuf/jsonpb [https://pkg.go.dev/cuelang.org/go/encoding/protobuf/jsonpb]

Last modified September 4, 2025 [https://github.com/cue-lang/cuelang.org/commit/c675de963f4124145b48e2681dab7b4aacab71e2]

 * encodings [/search?q=tag:encodings]
 * go api [/search?q=tag:%22go%20api%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-protocol-buffers/&text=Protocol%20Buffers,%20also%20known%20as%20Protobuf,%20is%20a%20language-neutral,%20platform-neutral,%20and%20extensible%20mechanism%20for%20serializing%20structured%20data,%20initially%20developed%20and%20released%20by%20Google.%0aProtobuf%20definitions%20can%20be%20converted%20to%20CUE%20by%20the%20cue%20command%20and%20CUE&rsquo;s%20Go%20API,%20promoting%20any%20CUE%20validation%20code%20placed%20in%20Protobuf%20options%20to%20first-class%20CUE%20value%20constraints.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-protocol-buffers/&summary=Protocol%20Buffers,%20also%20known%20as%20Protobuf,%20is%20a%20language-neutral,%20platform-neutral,%20and%20extensible%20mechanism%20for%20serializing%20structured%20data,%20initially%20developed%20and%20released%20by%20Google.%0aProtobuf%20definitions%20can%20be%20converted%20to%20CUE%20by%20the%20cue%20command%20and%20CUE&rsquo;s%20Go%20API,%20promoting%20any%20CUE%20validation%20code%20placed%20in%20Protobuf%20options%20to%20first-class%20CUE%20value%20constraints.%0a]


How CUE works with OpenAPI
[/docs/concept/how-cue-works-with-openapi/]How CUE works with TOML
[/docs/concept/how-cue-works-with-toml/]
 * Introduction [/docs/introduction/]
