---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#12-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 115
summary: `// Statusz is the config.k8s.io/v1alpha1 schema for the /statusz endpoint. type Statusz struct { // Kind is \"Statusz\". Kind string `json:\"kind\"` // APIVersion is the version of the object, e.g.,...
---

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