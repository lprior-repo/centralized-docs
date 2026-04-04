---
doc_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#3-standard
chunk_level: standard
chunk_type: prose
heading: Using this data
token_count: 427
summary: While the counter data looks like this: ``` `# HELP kubernetes\_healthchecks\_total [ALPHA] This metric records the results of all healthcheck. # TYPE kubernetes\_healthchecks\_total counter...
---

While the counter data looks like this:
```
`# HELP kubernetes\_healthchecks\_total [ALPHA] This metric records the results of all healthcheck.
# TYPE kubernetes\_healthchecks\_total counter
kubernetes\_healthchecks\_total{name="autoregister-completion",status="error",type="readyz"} 1
kubernetes\_healthchecks\_total{name="autoregister-completion",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="autoregister-completion",status="success",type="readyz"} 14
kubernetes\_healthchecks\_total{name="etcd",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="etcd",status="success",type="readyz"} 15
kubernetes\_healthchecks\_total{name="etcd-readiness",status="success",type="readyz"} 15
kubernetes\_healthchecks\_total{name="informer-sync",status="error",type="readyz"} 1
kubernetes\_healthchecks\_total{name="informer-sync",status="success",type="readyz"} 14
kubernetes\_healthchecks\_total{name="log",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="log",status="success",type="readyz"} 15
kubernetes\_healthchecks\_total{name="ping",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="ping",status="success",type="readyz"} 15
`
```
## Using this data
The component SLIs metrics endpoint is intended to be scraped at a high frequency. Scraping
at a high frequency means that you end up with greater granularity of the gauge's signal, which
can be then used to calculate SLOs. The `/metrics/slis` endpoint provides the raw data necessary
to calculate an availability SLO for the respective Kubernetes component.