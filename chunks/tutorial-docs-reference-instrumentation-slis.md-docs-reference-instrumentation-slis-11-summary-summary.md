---
doc_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#11-summary
chunk_level: summary
chunk_type: prose
heading: Using this data
token_count: 128
summary: kubernetes\_healthchecks\_total{name=\"ping\",status=\"success\",type=\"healthz\"} 15 kubernetes\_healthchecks\_total{name=\"ping\",status=\"success\",type=\"readyz\"} 15 ` ``` ## Using this data The component...
---

kubernetes\_healthchecks\_total{name="ping",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="ping",status="success",type="readyz"} 15
`
```
## Using this data
The component SLIs metrics endpoint is intended to be scraped at a high frequency. Scraping
at a high frequency means that you end up with greater granularity of the gauge's signal, which
can be then used to calculate SLOs. The `/metrics/slis` endpoint provides the raw data necessary
to calculate an availability SLO for the respective Kubernetes component.