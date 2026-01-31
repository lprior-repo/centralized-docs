---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#0-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1026
summary: # How CUE works with YAML | CUE. **Source:** https://cuelang
---

# How CUE works with YAML | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-yaml/

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


 2. HOW CUE WORKS WITH YAML

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

READING AND WRITING YAML

The cue tool natively supports reading and writing YAML files, including
those containing multiple documents.

This allows YAML files to be processed by CUE’s wide range of data, schema, and
policy validation capabilities, and to convert input formats to YAML - as
demonstrated here by
cue export [/docs/reference/command/cue-help-export/]
unifying all its YAML, JSON, and CUE input files as YAML:

 * 
   
   Copied!
   data.yml
 * 
   
   Copied!
   data.json
 * 
   
   Copied!
   data.cue

Copy code
Copied!

a: 1
b: "2"
c: "three"
d: 4.4


Copy code
Copied!

{
    "e": 5,
    "f": "6"
}


Copy code
Copied!

g: "seven"
h: 4.4 * 2

TERMINAL

Copy code
Copied!

$ cue export --out yaml data.yml data.json data.cue
a: 1
b: "2"
c: three
d: 4.4
e: 5
"f": "6"
g: seven
h: 8.8

In addition to YAML, cue can read and write
a range of other formats [/docs/integration/].

VALIDATING YAML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate YAML files
using CUE’s powerful and compact constraint syntax, it’s easy to add
“pre-flight” checks to existing processes with CUE.

In this example,
cue vet [/docs/reference/command/cue-help-vet/]
is used to check that a hypothetical system’s YAML input files are valid - and
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
   config-a.yaml
 * 
   
   Copied!
   config-b.yaml
 * 
   
   Copied!
   config-c.yaml

Copy code
Copied!

cluster: live05
region: IMEA
repository: source.company.example/alpha
tags:
  - prod


Copy code
Copied!

cluster: live03333333333333
repository: github.com/Alex_Personal_Account/alpha-fork
region: UK
tags:
  - dev


Copy code
Copied!

cluster: live05
region: APAC
repository: source.company.example/alpha

TERMINAL

Copy code
Copied!

$ cue vet -c schema.cue -d '#Config' config-a.yaml config-b.yaml config-c.yaml
region: 2 errors in empty disjunction:
region: conflicting values "APAC" and "UK":
    ./config-b.yaml:3:9
    ./schema.cue:5:15
    ./schema.cue:9:10
region: conflicting values "IMEA" and "UK":
    ./config-b.yaml:3:9
    ./schema.cue:5:15
    ./schema.cue:9:19
cluster: invalid value "live03333333333333" (does not satisfy strings.MaxRunes(16)):
    ./schema.cue:4:15
    ./config-b.yaml:1:10
    ./schema.cue:4:32
repository: invalid value "github.com/Alex_Personal_Account/alpha-fork" (out of bound =~"^source\\.company\\.example/"):
    ./schema.cue:6:15
    ./config-b.yaml:2:13

Learn more in the How-to guide Validating YAML using CUE [/docs/howto/validate-yaml-using-cue/].

PROCESSING AND TRANSFORMING YAML FILES

The cue tool can read and transform YAML files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
