---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#24-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 103
summary: // Flags contains the command-line flags and their values. // The keys are the flag names and the values are the flag values, // possibly with confidential values redacted. // +optional Flags...
---

// Flags contains the command-line flags and their values.
// The keys are the flag names and the values are the flag values,
// possibly with confidential values redacted.
// +optional
Flags map[string]string `json:"flags,omitempty"`
}
`
```
#### Note:
The structured responses for both `/statusz` and `/flagz` are alpha features in v1.35
and are subject to change in future releases.
They are intended to provide machine-parseable output for debugging and introspection tools.