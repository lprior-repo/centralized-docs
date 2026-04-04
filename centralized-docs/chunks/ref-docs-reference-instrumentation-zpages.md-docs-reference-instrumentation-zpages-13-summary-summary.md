---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#13-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 116
summary: Metadata metav1.ObjectMeta `json:\"metadata,omitempty\"` // StartTime is the time the component process was initiated. StartTime metav1.Time `json:\"startTime\"` // UptimeSeconds is the duration in...
---

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