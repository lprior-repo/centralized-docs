---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#15-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 85
summary: // MinimumCompatibilityVersion is the minimum Kubernetes API version with which the component is designed to work. // if present, formatted as \"&lt;major&gt;.&lt;minor&gt;\" // +optional...
---

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