---
doc_id: docs-concept-how-cue-works-with-json
chunk_id: docs-concept-how-cue-works-with-json#0
chunk_type: table
heading: Introduction
token_count: 3775
summary: # How CUE works with JSON | CUE. **Source:** https://cuelang
---

# How CUE works with JSON | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-json/

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


 2. HOW CUE WORKS WITH JSON

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

READING AND WRITING JSON

CUE is a superset of JSON [/docs/tour/basics/json-superset/].
In other words: all valid JSON is CUE.

The cue tool natively supports reading and writing JSON files. In fact, JSON
is its default output format.

This allows JSON files to be processed by CUE’s wide range of data, schema, and
policy validation capabilities, and to convert input formats to JSON - as
demonstrated here by
cue export [/docs/reference/command/cue-help-export/]
unifying all its JSON, YAML, and CUE input files as JSON:

 * 
   
   Copied!
   data.json
 * 
   
   Copied!
   data.yml
 * 
   
   Copied!
   data.cue

Copy code
Copied!

{
    "a": 1,
    "b": "2",
    "c": "three",
    "d": 4.4
}


Copy code
Copied!

e: 5
f: "6"


Copy code
Copied!

g: "seven"
h: 4.4 * 2

TERMINAL

Copy code
Copied!

$ cue export data.json data.yml data.cue
{
    "a": 1,
    "b": "2",
    "c": "three",
    "d": 4.4,
    "e": 5,
    "f": "6",
    "g": "seven",
    "h": 8.8
}

In addition to JSON, cue can read and write
a range of other formats [/docs/integration/].

VALIDATING JSON FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate JSON files
using CUE’s powerful and compact constraint syntax, it’s easy to add
“pre-flight” checks to existing processes with CUE.

In this example,
cue vet [/docs/reference/command/cue-help-vet/]
is used to check that a hypothetical system’s JSON input files are valid - and
catches a problematic deployment early in the process:

Copied!
schema.cue

Copy code
Copied!

import "strings"

#Config: {
	cluster!:    strings.MaxRunes(16)
	region!:     #Region
	repository!: =~#"^source\.company\.example/"#
	tags?: [...#Tags]
}
#Region: "APAC" | "IMEA"
#Tags:   "prod" | "stage" | "qa" | "test" | "dev"

 * 
   
   Copied!
   config-a.json
 * 
   
   Copied!
   config-b.json
 * 
   
   Copied!
   config-c.json

Copy code
Copied!

{
    "cluster": "live05",
    "region": "IMEA",
    "repository": "source.company.example/alpha",
    "tags": [
        "prod"
    ]
}


Copy code
Copied!

{
    "cluster": "live03333333333333",
    "repository": "github.com/Alex_Personal_Account/alpha-fork",
    "region": "UK",
    "tags": [
        "dev"
    ]
}


Copy code
Copied!

{
    "cluster": "live05",
    "region": "APAC",
    "repository": "source.company.example/alpha"
}

TERMINAL

Copy code
Copied!

$ cue vet -c schema.cue -d '#Config' config-a.json config-b.json config-c.json
region: 2 errors in empty disjunction:
region: conflicting values "APAC" and "UK":
    ./config-b.json:4:15
    ./schema.cue:5:15
    ./schema.cue:9:10
region: conflicting values "IMEA" and "UK":
    ./config-b.json:4:15
    ./schema.cue:5:15
    ./schema.cue:9:19
cluster: invalid value "live03333333333333" (does not satisfy strings.MaxRunes(16)):
    ./schema.cue:4:15
    ./config-b.json:2:16
    ./schema.cue:4:32
repository: invalid value "github.com/Alex_Personal_Account/alpha-fork" (out of bound =~"^source\\.company\\.example/"):
    ./schema.cue:6:15
    ./config-b.json:3:19

Learn more in the How-to guide Validating JSON using CUE [/docs/howto/validate-json-using-cue/].

PROCESSING AND TRANSFORMING JSON FILES

The cue tool can read and transform JSON files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.json

Copy code
Copied!

{
    "a": 5,
    "b": 4
}

TERMINAL

Copy code
Copied!

$ cue export data.json transform.cue
{
    "a": 5,
    "b": 4,
    "c": 21
}

Learn more about transforming data with CUE in these How-to guides:

 * Transforming JSON with CUE [/docs/howto/transform-json-with-cue/]
 * Combining multiple JSON files into a list [/docs/howto/combine-multiple-json-files-into-a-list/]
 * Combining multiple JSON files by using file metadata [/docs/howto/combine-multiple-json-files-by-using-file-metadata/]

EMBEDDING JSON FILE DATA INSIDE CUE

Requires CUE v0.12.0 or later

The file embedding [/docs/howto/embed-files-in-cue-evaluation/]
feature allows data files (including JSON) to be read when some CUE is evaluated.
This provides an alternative way to use CUE to validate data files against
schemas and constraints, and also gives CUE configurations access to data
stored in non-CUE files:

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.json)
_data: _ @embed(glob=env/*.json)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).json"].text
}

 * 
   
   Copied!
   config.json
 * 
   
   Copied!
   env/foo.json
 * 
   
   Copied!
   env/bar.json

Copy code
Copied!

{
    "version": "1.42.0",
    "source": "bar"
}


Copy code
Copied!

{
    "text": "Some foo"
}


Copy code
Copied!

{
    "text": "A bar"
}

TERMINAL

Copy code
Copied!

$ cue export
{
    "info": {
        "version": "1.42.0",
        "source": "A bar"
    }
}

File embedding is available from CUE v0.12.0 onwards.
Find out more about this powerful validation feature in
Embedding files in a CUE evaluation [/docs/howto/embed-files-in-cue-evaluation/].

ENCODING JSON INSIDE CUE

CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain JSON encoded in string fields,
irrespective of the file’s main data format.

CUE’s standard library provides
a built-in json package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/json]
containing functions that generate, parse, validate, and format JSON from
within CUE - some of which are shown here.

GENERATING ENCODED JSON

In this example a Kubernetes ConfigMap contains a JSON file encoded as a
single string field, in a YAML document. This is enabled by the
json.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/json"

configMap: data: "point.json": json.Marshal({
	x: 1.2
	y: 3.45
})

TERMINAL

Copy code
Copied!

$ cue export config.cue --out yaml
configMap:
  data:
    point.json: '{"x":1.2,"y":3.45}'

PARSING ENCODED JSON

The json.Unmarshal function performs the reverse operation to json.Marshal:
it turns a string containing JSON into the structure represented by
the encoded data.

Here, some encoded JSON data (a JSON Web Token) is emitted as YAML:

Copied!
token.cue

Copy code
Copied!

import "encoding/json"

_jwt: {
	header: #"{"alg":"HS256","typ":"JWT"}"#
	payload: """
		{
		  "sub": "1234567890",
		  "name": "John Doe",
		  "iat": 1516239022
		}
		"""
}
output: header:  json.Unmarshal(_jwt.header)
output: payload: json.Unmarshal(_jwt.payload)

TERMINAL

Copy code
Copied!

$ cue export token.cue --out yaml
output:
  header:
    alg: HS256
    typ: JWT
  payload:
    sub: "1234567890"
    name: John Doe
    iat: 1516239022

VALIDATING ENCODED JSON

The json.Validate function allows encoded JSON to be validated against
native CUE schema constraints.

Here, each member of the item map is checked against the #Dimensions
schema. The cue tool correctly catches and flags up two problems with the
data:

Copied!
furniture.cue

Copy code
Copied!

import "encoding/json"

#Dimensions: {
	width:  number
	depth:  number
	height: number
}

// Validate each member of the map against a schema.
item: [string]: json.Validate(#Dimensions)

// bed is correctly specified.
item: bed: #"{ "width": 2, "height": 0.1, "depth": 2 }"#
// table's width is incorrectly specified as a string.
item: table: #"{ "width": "34", "height": 23, "depth": 0.2 }"#
// painting's height field name is incorrectly upper-cased.
item: painting: #"{ "width": 34, "HEIGHT": 12, "depth": 0.2 }"#

TERMINAL

Copy code
Copied!

$ cue vet -c furniture.cue
item.painting: invalid value "{ \"width\": 34, \"HEIGHT\": 12, \"depth\": 0.2 }" (does not satisfy encoding/json.Validate): error in call to encoding/json.Validate: field not allowed:
    ./furniture.cue:10:17
    ./furniture.cue:17:17
    json.Validate:1:16
item.table: invalid value "{ \"width\": \"34\", \"height\": 23, \"depth\": 0.2 }" (does not satisfy encoding/json.Validate): error in call to encoding/json.Validate: conflicting values "34" and number (mismatched types string and number):
    ./furniture.cue:10:17
    ./furniture.cue:4:10
    ./furniture.cue:15:14
    json.Validate:1:1
    json.Validate:1:12

OTHER JSON FUNCTIONS

The
json package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/json]
contains other useful functions, including those that format JSON specifically
for humans to read, or for machines to consume. These functions are
demonstrated in guides that you can discover through the site’s search page:
🔍 
search for how-to guides mentioning “encoding/json” [/search/?q=encoding/json%20contentType:%22How-to%20Guides%22]

CONVERTING JSON FILES TO CUE

Because
every valid JSON file is also a CUE file [/docs/tour/basics/json-superset/],
one very easy way to convert JSON files to CUE is simply to rename them from
.json to .cue!

In more complex situations
cue import [/docs/reference/command/cue-help-import/]
can create a CUE file for each JSON file it’s given, and can even recognise
encoded YAML and JSON fields, and convert those structures recursively.

Examples of this command being used can be found in the
cue import CLI reference documentation [/docs/reference/command/cue-help-import/].

RELATED CONTENT

 * Concept Guide: How CUE works with YAML [/docs/concept/how-cue-works-with-yaml/]
 * Concept Guide: How CUE works with TOML [/docs/concept/how-cue-works-with-toml/]

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-json/&text=%20Reading%20and%20writing%20JSON%20CUE%20is%20a%20superset%20of%20JSON.%20In%20other%20words:%20all%20valid%20JSON%20is%20CUE.%0aThe%20cue%20tool%20natively%20supports%20reading%20and%20writing%20JSON%20files.%20In%20fact,%20JSON%20is%20its%20default%20output%20format.%0aThis%20allows%20JSON%20files%20to%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities,%20and%20to%20convert%20input%20formats%20to%20JSON%20-%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20all%20its%20JSON,%20YAML,%20and%20CUE%20input%20files%20as%20JSON:%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-json/&summary=%20Reading%20and%20writing%20JSON%20CUE%20is%20a%20superset%20of%20JSON.%20In%20other%20words:%20all%20valid%20JSON%20is%20CUE.%0aThe%20cue%20tool%20natively%20supports%20reading%20and%20writing%20JSON%20files.%20In%20fact,%20JSON%20is%20its%20default%20output%20format.%0aThis%20allows%20JSON%20files%20to%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities,%20and%20to%20convert%20input%20formats%20to%20JSON%20-%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20all%20its%20JSON,%20YAML,%20and%20CUE%20input%20files%20as%20JSON:%0a]


How CUE works with Go
[/docs/concept/how-cue-works-with-go/]How CUE works with JSON Schema
[/docs/concept/how-cue-works-with-json-schema/]
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
   * How CUE works with JSON [/docs/concept/how-cue-works-with-json/]
      1. Reading and writing JSON
      2. Validating JSON files against a schema
      3. Processing and transforming JSON files
      4. Embedding JSON file data inside CUE
      5. Encoding JSON inside CUE
      6. Converting JSON files to CUE
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fhow-cue-works-with-json%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
