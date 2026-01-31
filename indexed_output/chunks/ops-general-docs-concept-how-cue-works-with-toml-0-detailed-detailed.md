---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#0-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1026
summary: # How CUE works with TOML | CUE. **Source:** https://cuelang
---

# How CUE works with TOML | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-toml/

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


 2. HOW CUE WORKS WITH TOML

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

READING AND WRITING TOML

The cue command natively supports reading and writing TOML files and data.
TOML can be processed by CUE’s wide range of data, schema, and policy
validation capabilities.
Data in any supported encoding can be read and exported as TOML
– as demonstrated here by
cue export [/docs/reference/command/cue-export/]
unifying its TOML, JSON, and CUE input files and producing TOML:

 * 
   
   Copied!
   a.toml
 * 
   
   Copied!
   b.json
 * 
   
   Copied!
   c.cue

Copy code
Copied!

a = "1"

[b]
c = 2.2

[b.d]
e = 3


Copy code
Copied!

{
    "f": "4",
    "g": 5.5
}


Copy code
Copied!

b: _
g: _

h: "six"
b: d: i: g + b.d.e

TERMINAL

Copy code
Copied!

$ cue export --out toml a.toml b.json c.cue
a = '1'
f = '4'
g = 5.5
h = 'six'

[b]
c = 2.2

[b.d]
e = 3
i = 8.5

The cue command can read and write
a range of other formats [/docs/integration/]
as well as TOML.

VALIDATING TOML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate TOML files
using CUE’s powerful and compact constraint syntax, it’s easy to add
“pre-flight” checks to existing processes with CUE.

In this example,
cue vet [/docs/reference/command/cue-help-vet/]
is used to check that a hypothetical system’s TOML input files are valid - and
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
   config-a.toml
 * 
   
   Copied!
   config-b.toml
 * 
   
   Copied!
   config-c.toml

Copy code
Copied!

cluster = 'live05'
region = 'IMEA'
repository = 'source.company.example/alpha'
tags = ['prod']


Copy code
Copied!

cluster = 'live03333333333333'
region = 'UK'
repository = 'github.com/Alex_Personal_Account/alpha-fork'
tags = ['dev']


Copy code
Copied!

cluster = 'live05'
region = 'APAC'
repository = 'source.company.example/alpha'

TERMINAL

Copy code
Copied!

$ cue vet -c schema.cue -d '#Config' config-a.toml config-b.toml config-c.toml
region: 2 errors in empty disjunction:
region: conflicting values "APAC" and "UK":
    ./config-b.toml:2:10
    ./schema.cue:5:15
    ./schema.cue:9:10
region: conflicting values "IMEA" and "UK":
    ./config-b.toml:2:10
    ./schema.cue:5:15
    ./schema.cue:9:19
cluster: invalid value "live03333333333333" (does not satisfy strings.MaxRunes(16)):
    ./schema.cue:4:15
    ./config-b.toml:1:11
    ./schema.cue:4:32
repository: invalid value "github.com/Alex_Personal_Account/alpha-fork" (out of bound =~"^source\\.company\\.example/"):
    ./schema.cue:6:15
    ./config-b.toml:3:14

PROCESSING AND TRANSFORMING TOML FILES

The cue tool can read and transform TOML files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.toml

Copy code
