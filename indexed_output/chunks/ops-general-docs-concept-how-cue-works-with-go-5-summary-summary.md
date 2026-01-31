---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#5-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 128
summary: mod/gen/k8s. |   |-- apps
---

cue.mod/gen/k8s.io
|-- api
|   |-- apps
|   |   `-- v1
|   `-- core
|       `-- v1
`-- apimachinery
    `-- pkg
        |-- api
        |   `-- resource
        |-- apis
        |   `-- meta
        |       `-- v1
...

cue get go [/docs/reference/command/cue-help-get-go/]
also has a --local option that generates CUE alongside Go in a main module.

Within our main module, we can import and refer to the CUE definitions generated from the Go types:

Copied!
config.cue

Copy code
Copied!

package config

import (
