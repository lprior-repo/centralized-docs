---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 121
summary: # Kubernetes z-pages Provides runtime diagnostics for Kubernetes components, offering insights into component runtime status and configuration flags. FEATURE STATE: `Kubernetes v1.32 [alpha]`...
---

# Kubernetes z-pages
Provides runtime diagnostics for Kubernetes components, offering insights into component runtime status and configuration flags.
FEATURE STATE:
`Kubernetes v1.32 [alpha]`
Kubernetes core components can expose a suite of *z-endpoints* to make it easier for users
to debug their cluster and its components. These endpoints are strictly to be used for human
inspection to gain real time debugging information of a component binary.
Avoid automated scraping of data returned by these endpoints; in Kubernetes 1.35
these are an **alpha** feature and the response format may change in future releases.