---
doc_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#4-summary
chunk_level: summary
chunk_type: prose
heading: SLI Metrics
token_count: 111
summary: `# HELP kubernetes\_healthcheck [ALPHA] This metric records the result of a single healthcheck. # TYPE kubernetes\_healthcheck gauge...
---

`# HELP kubernetes\_healthcheck [ALPHA] This metric records the result of a single healthcheck.
# TYPE kubernetes\_healthcheck gauge
kubernetes\_healthcheck{name="autoregister-completion",type="healthz"} 1
kubernetes\_healthcheck{name="autoregister-completion",type="readyz"} 1
kubernetes\_healthcheck{name="etcd",type="healthz"} 1
kubernetes\_healthcheck{name="etcd",type="readyz"} 1