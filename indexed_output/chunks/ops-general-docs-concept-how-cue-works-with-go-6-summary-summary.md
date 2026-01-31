---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: io/api/core/v1\". io/api/apps/v1\"
---

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
