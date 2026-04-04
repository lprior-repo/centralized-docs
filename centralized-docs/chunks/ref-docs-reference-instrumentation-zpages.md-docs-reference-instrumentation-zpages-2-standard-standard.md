---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#2-standard
chunk_level: standard
chunk_type: prose
heading: z-pages
token_count: 444
summary: ## z-pages Kubernetes v1.35 allows you to enable *z-pages* to help you troubleshoot problems with its core control plane components. These special debugging endpoints provide internal information...
---

## z-pages
Kubernetes v1.35 allows you to enable *z-pages* to help you troubleshoot
problems with its core control plane components. These special debugging endpoints provide internal
information about running components. For Kubernetes 1.35, components
serve the following endpoints (when enabled):
* [z-pages](#z-pages)
* [statusz](#statusz)
* [statusz (structured)](#statusz-structured)
* [flagz](#flagz)
* [flagz (structured)](#flagz-structured)### statusz
Enabled using the `ComponentStatusz` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ComponentStatusz),
the `/statusz` endpoint displays high level information about the component such as its Kubernetes version, emulation version, start time and more.
The `/statusz` plain text response from the API server is similar to:
```
`kube-apiserver statusz
Warning: This endpoint is not meant to be machine parseable, has no formatting compatibility guarantees and is for debugging purposes only.
Started: Wed Oct 16 21:03:43 UTC 2024
Up: 0 hr 00 min 16 sec
Go version: go1.23.2
Binary version: 1.32.0-alpha.0.1484&amp;#43;5eeac4f21a491b-dirty
Emulation version: 1.32.0-alpha.0.1484
Paths: /healthz /livez /metrics /readyz /statusz /version
`
```
#### statusz (structured)
FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Starting with Kubernetes v1.35, the `/statusz` endpoint supports a structured,
versioned response format when requested with the appropriate `Accept` header.
Without an `Accept` header, the endpoint returns the plain text response format by default.
To request the structured response, use:
```
`Accept: application/json;v=v1alpha1;g=config.k8s.io;as=Statusz
`
```