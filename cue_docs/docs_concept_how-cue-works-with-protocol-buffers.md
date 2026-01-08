# How CUE works with Protocol Buffers | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-protocol-buffers/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. Concept Guides [https://cuelang.org/docs/concept/]


 2. HOW CUE WORKS WITH PROTOCOL BUFFERS

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
 * encodings [/search?q=tag:encodings]
 * go api [/search?q=tag:%22go%20api%22]

Protocol Buffers [https://protobuf.dev/], also known as Protobuf, is a
language-neutral, platform-neutral, and extensible mechanism for serializing
structured data, initially developed and released by Google.

Protobuf definitions can be converted to CUE by the cue command and CUE’s Go
API, promoting any CUE validation code placed in Protobuf options to
first-class CUE value constraints.

USING THE CUE COMMAND

Let’s start by converting Protobuf to CUE using the cue command.
We’ll begin with this Protobuf file, basic.proto:

Copied!
basic.proto

Copy code
Copied!

syntax = "proto3";

// Package basic is rather basic.
package cuelang.examples.basic;

import "cue/cue.proto";

option go_package = "cuelang.org/encoding/protobuf/examples/basic";

// This is my type.
message MyType {
    string string_value = 1; // Some string value

    // A method must start with a capital letter.
    repeated string method = 2 [(cue.val) = '[...=~"^[A-Z]"]'];
}

The cue import command converts Protobuf to CUE.
It indicates success by displaying no output:

TERMINAL

Copy code
Copied!

$ cue import basic.proto

The command creates this CUE file:

Copied!
basic.cue

Copy code
Copied!

// Package basic is rather basic.
package basic

// This is my type.
#MyType: {
	stringValue?: string @protobuf(1,string,name=string_value) // Some string value

	// A method must start with a capital letter.
	method?: [...string] @protobuf(2,string)
	method?: [...=~"^[A-Z]"]
}

Do you notice how the method field’s type and constraints are split over two lines?
That’s exactly as expected, because CUE’s core operation is to automatically
unify each field’s right-hand-side.

The behaviour of the cue import command can be affected by the flags outlined
in the proto mode section of
the command’s help text [/docs/reference/command/cue-help-import/].

USING THE GO API

CUE’s Go API can achieve the same result as the cue import command,
converting Protobuf definitions to CUE, but with
more customization and flexibility [https://pkg.go.dev/cuelang.org/go/encoding/protobuf#Config].

This simple Go code takes the basic.proto file shown above, and prints the
equivalent CUE:

Copied!
main.go

Copy code
Copied!

package main

import (
	"fmt"
	"log"

	"cuelang.org/go/cue/format"
	"cuelang.org/go/encoding/protobuf"
)

func main() {
	file, err := protobuf.Extract("basic.proto", nil, &protobuf.Config{
		Paths: []string{ /* paths to proto includes */ },
	})
	if err != nil {
		log.Fatal(err)
	}
	b, _ := format.Node(file)
	fmt.Println(string(b))
}

Here’s the program’s output:

TERMINAL

Copy code
Copied!

$ go run main.go
// Package basic is rather basic.
package basic

// This is my type.
#MyType: {
	stringValue?: string @protobuf(1,string,name=string_value) // Some string value

	// A method must start with a capital letter.
	method?: [...string] @protobuf(2,string)
	method?: [...=~"^[A-Z]"]
}

Notice that the ouput is identical to the CUE produced previously by cue import.

EXTRACTING CUE FROM SEVERAL PROTOBUF FILES

In some environments it might be necessary to import multiple Protobuf files
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
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
   * Popular guides [/docs/concept/popular-guides/]
   * The Logic of CUE [/docs/concept/the-logic-of-cue/]
   * Modules [/docs/concept/modules/]
   * Frequently Asked Questions [/docs/concept/faq/]
   * How CUE works with Protocol Buffers [/docs/concept/how-cue-works-with-protocol-buffers/]
      1. Using the cue command
      2. Using the Go API
      3. Extracting CUE from several Protobuf files
      4. Experimental APIs
      5. Protobuf mappings
      6. Future plans
      7. Related content
 * References [/docs/reference/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fhow-cue-works-with-protocol-buffers%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]