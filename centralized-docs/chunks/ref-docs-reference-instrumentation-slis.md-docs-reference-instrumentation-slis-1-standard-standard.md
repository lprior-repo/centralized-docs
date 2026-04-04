---
doc_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: # Kubernetes Component SLI Metrics High-level indicators for measuring the reliability and performance of Kubernetes components. FEATURE STATE: `Kubernetes v1.32 [stable]`(enabled by default) By...
---

# Kubernetes Component SLI Metrics
High-level indicators for measuring the reliability and performance of Kubernetes components.
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
By default, Kubernetes 1.35 publishes Service Level Indicator (SLI) metrics
for each Kubernetes component binary. This metric endpoint is exposed on the serving
HTTPS port of each component, at the path `/metrics/slis`. The
`ComponentSLIs` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
defaults to enabled for each Kubernetes component as of v1.27.