---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#1-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 518
summary: To demonstrate this, we’re going to fetch some Go source code published by the. Kubernetes project, import some types it defines, and use some of the CUE that
---

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
