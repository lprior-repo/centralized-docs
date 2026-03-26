---
doc_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#2-standard
chunk_level: standard
chunk_type: prose
heading: SLI Metrics
token_count: 347
summary: ## SLI Metrics With SLI metrics enabled, each Kubernetes component exposes two metrics, labeled per healthcheck: * a gauge (which represents the current state of the healthcheck) * a counter (which...
---

## SLI Metrics
With SLI metrics enabled, each Kubernetes component exposes two metrics,
labeled per healthcheck:
* a gauge (which represents the current state of the healthcheck)
* a counter (which records the cumulative counts observed for each healthcheck state)
You can use the metric information to calculate per-component availability statistics.
For example, the API server checks the health of etcd. You can work out and report how
available or unavailable etcd has been - as reported by its client, the API server.
The prometheus gauge data looks like this:
```
`# HELP kubernetes\_healthcheck [ALPHA] This metric records the result of a single healthcheck.
# TYPE kubernetes\_healthcheck gauge
kubernetes\_healthcheck{name="autoregister-completion",type="healthz"} 1
kubernetes\_healthcheck{name="autoregister-completion",type="readyz"} 1
kubernetes\_healthcheck{name="etcd",type="healthz"} 1
kubernetes\_healthcheck{name="etcd",type="readyz"} 1
kubernetes\_healthcheck{name="etcd-readiness",type="readyz"} 1
kubernetes\_healthcheck{name="informer-sync",type="readyz"} 1
kubernetes\_healthcheck{name="log",type="healthz"} 1
kubernetes\_healthcheck{name="log",type="readyz"} 1
kubernetes\_healthcheck{name="ping",type="healthz"} 1
kubernetes\_healthcheck{name="ping",type="readyz"} 1
`
```
While the counter data looks like this: