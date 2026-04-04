---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#14-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 122
summary: // +optional GoVersion string `json:\"goVersion,omitempty\"` // BinaryVersion is the version of the component's binary. // The format is not guaranteed to be semantic versioning and may be an arbitrary...
---

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