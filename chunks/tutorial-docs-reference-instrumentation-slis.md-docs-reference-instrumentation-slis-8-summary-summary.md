---
doc_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#8-summary
chunk_level: summary
chunk_type: prose
heading: SLI Metrics
token_count: 118
summary: `# HELP kubernetes\_healthchecks\_total [ALPHA] This metric records the results of all healthcheck. # TYPE kubernetes\_healthchecks\_total counter...
---

`# HELP kubernetes\_healthchecks\_total [ALPHA] This metric records the results of all healthcheck.
# TYPE kubernetes\_healthchecks\_total counter
kubernetes\_healthchecks\_total{name="autoregister-completion",status="error",type="readyz"} 1
kubernetes\_healthchecks\_total{name="autoregister-completion",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="autoregister-completion",status="success",type="readyz"} 14