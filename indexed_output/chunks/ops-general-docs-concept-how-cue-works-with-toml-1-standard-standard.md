---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#1-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 514
summary: using CUE’s powerful and compact constraint syntax, it’s easy to add. “pre-flight” checks to existing processes with CUE
---

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
Copied!

a = 5
b = 4

TERMINAL

Copy code
