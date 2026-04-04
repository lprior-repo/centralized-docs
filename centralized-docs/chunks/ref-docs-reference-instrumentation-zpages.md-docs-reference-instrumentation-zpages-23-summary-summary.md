---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#23-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 121
summary: `// Flagz is the config.k8s.io/v1alpha1 schema for the /flagz endpoint. type Flagz struct { // Kind is \"Flagz\". Kind string `json:\"kind\"` // APIVersion is the version of the object, e.g.,...
---

`// Flagz is the config.k8s.io/v1alpha1 schema for the /flagz endpoint.
type Flagz struct {
// Kind is "Flagz".
Kind string `json:"kind"`
// APIVersion is the version of the object, e.g., "config.k8s.io/v1alpha1".
APIVersion string `json:"apiVersion"`
// Standard object's metadata.
// +optional
Metadata metav1.ObjectMeta `json:"metadata,omitempty"`
// Flags contains the command-line flags and their values.
// The keys are the flag names and the values are the flag values,