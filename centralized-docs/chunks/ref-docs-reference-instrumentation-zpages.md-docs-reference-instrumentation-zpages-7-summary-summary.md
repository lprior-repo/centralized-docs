---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#7-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 110
summary: #### statusz (structured) FEATURE STATE: `Kubernetes v1.32 [alpha]`(disabled by default) Starting with Kubernetes v1.35, the `/statusz` endpoint supports a structured, versioned response format when...
---

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