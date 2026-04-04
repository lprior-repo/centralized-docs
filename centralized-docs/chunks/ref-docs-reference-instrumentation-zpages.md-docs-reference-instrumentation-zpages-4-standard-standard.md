---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#4-standard
chunk_level: standard
chunk_type: prose
heading: z-pages
token_count: 390
summary: The `config.k8s.io/v1alpha1` schema for the structured `/statusz` response is as follows: ``` `// Statusz is the config.k8s.io/v1alpha1 schema for the /statusz endpoint. type Statusz struct { // Kind...
---

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