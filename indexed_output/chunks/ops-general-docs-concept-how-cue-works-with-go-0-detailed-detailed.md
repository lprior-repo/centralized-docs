---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#0-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1032
summary: # How CUE works with Go | CUE. **Source:** https://cuelang
---

# How CUE works with Go | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-go/

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


 2. HOW CUE WORKS WITH GO

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
myitcv [https://github.com/myitcv.png]
Paul Jolly
myitcv [https://github.com/myitcv.png]
Paul Jolly

Github profile

[https://github.com/myitcv]

Search all content by this author

[/search/?q=author:myitcv]
 * encodings [/search?q=tag:encodings]
 * go api [/search?q=tag:%22go%20api%22]

CUE is designed to complement and work with the Go programming language.
It offers a powerful API that enables Go code to take advantage of CUE’s
advanced capabilites.
Additionally, CUE makes it easy to use Go as your source of truth by using the
cue command to convert Go types to CUE.

In this guide we’ll demonstrate importing some Kubernetes API code to generate
CUE schemas. We’ll also use the API to convert both CUE and non-CUE data to
native Go values, and validate some Go data natively with CUE.

CONVERTING GO TYPES TO CUE

If you’ve already invested time in developing Go types, you might need them to
be the source of truth in your system whilst also wanting to validate data that
matches those types against the more detailed constraints that CUE allows.

The cue command can help you achieve this as it can convert arbitrary Go types to CUE.
To demonstrate this, we’re going to fetch some Go source code published by the
Kubernetes project, import some types it defines, and use some of the CUE that
gets produced.

Let’s start by downloading a specific version of the k8s.io/api module:

TERMINAL

Copy code
Copied!

$ go get k8s.io/api/apps/v1@v0.29.3
...

We use cue get go to generate CUE definitions from the Go types in the k8s.io/api/apps/v1 package:

TERMINAL

Copy code
Copied!

$ cue get go k8s.io/api/apps/v1

This generates some CUE packages, placing them alongside our main CUE module:

TERMINAL

Copy code
Copied!

$ tree -d cue.mod/gen/k8s.io
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
	core "k8s.io/api/core/v1"
	apps "k8s.io/api/apps/v1"
)

service: [string]:     core.#Service
deployment: [string]:  apps.#Deployment
daemonSet: [string]:   apps.#DaemonSet
statefulSet: [string]: apps.#StatefulSet

Our configuration is currently empty - but any
services, deployments, daemonSets, or statefulSets
that we add will be checked against the schema of the associated Kubernetes type:

TERMINAL

Copy code
Copied!

$ cue eval
service: {}
deployment: {}
daemonSet: {}
statefulSet: {}

A more in-depth example demonstrating how to drive Kubernetes configuration
using CUE can be found in CUE By Example, in
Controlling Kubernetes with CUE [https://github.com/cue-labs/cue-by-example/blob/main/003_kubernetes_tutorial/README.md].

The example above relies on generating CUE within the cue.mod/gen directory
of the CUE module that holds a configuration,
but we are working on a system for providing schemas for well-known services at
