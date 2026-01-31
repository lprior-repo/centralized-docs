---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#1-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 514
summary: VALIDATING JSON FILES AGAINST A SCHEMA. CUE is often used to make systems safer without having to teach the underlying
---


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
