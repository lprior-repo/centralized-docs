---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#2-detailed
chunk_level: detailed
chunk_type: prose
heading: z-pages
token_count: 1002
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
#### Note:
If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`),
the server will respond with `406 Not Acceptable`.
Example structured response:
```
`{
"kind": "Statusz",
"apiVersion": "config.k8s.io/v1alpha1",
"metadata": {
"name": "kube-apiserver"
},
"startTime": "2025-10-29T00:30:01Z",
"uptimeSeconds": 856,
"goVersion": "go1.23.2",
"binaryVersion": "1.35.0",
"emulationVersion": "1.35",
"paths": [
"/healthz",
"/livez",
"/metrics",
"/readyz",
"/statusz",
"/version"
]
}
`
```
The `config.k8s.io/v1alpha1` schema for the structured `/statusz` response is as follows:
```
`// Statusz is the config.k8s.io/v1alpha1 schema for the /statusz endpoint.
type Statusz struct {
// Kind is "Statusz".
Kind string `json:"kind"`
// APIVersion is the version of the object, e.g., "config.k8s.io/v1alpha1".
APIVersion string `json:"apiVersion"`
// Standard object's metadata.
// +optional
Metadata metav1.ObjectMeta `json:"metadata,omitempty"`
// StartTime is the time the component process was initiated.
StartTime metav1.Time `json:"startTime"`
// UptimeSeconds is the duration in seconds for which the component has been running continuously.
UptimeSeconds int64 `json:"uptimeSeconds"`
// GoVersion is the version of the Go programming language used to build the binary.
// The format is not guaranteed to be consistent across different Go builds.
// +optional
GoVersion string `json:"goVersion,omitempty"`
// BinaryVersion is the version of the component's binary.
// The format is not guaranteed to be semantic versioning and may be an arbitrary string.
BinaryVersion string `json:"binaryVersion"`
// EmulationVersion is the Kubernetes API version which this component is emulating.
// if present, formatted as "&lt;major&gt;.&lt;minor&gt;"
// +optional
EmulationVersion string `json:"emulationVersion,omitempty"`
// MinimumCompatibilityVersion is the minimum Kubernetes API version with which the component is designed to work.
// if present, formatted as "&lt;major&gt;.&lt;minor&gt;"
// +optional
MinimumCompatibilityVersion string `json:"minimumCompatibilityVersion,omitempty"`
// Paths contains relative URLs to other essential read-only endpoints for debugging and troubleshooting.
// +optional
Paths []string `json:"paths,omitempty"`
}
`
```