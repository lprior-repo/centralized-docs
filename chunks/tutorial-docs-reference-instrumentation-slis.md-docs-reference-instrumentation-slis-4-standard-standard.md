---
doc_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: tutorial/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#4-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 422
summary: ## Using this data The component SLIs metrics endpoint is intended to be scraped at a high frequency. Scraping at a high frequency means that you end up with greater granularity of the gauge's...
---

## Using this data
The component SLIs metrics endpoint is intended to be scraped at a high frequency. Scraping
at a high frequency means that you end up with greater granularity of the gauge's signal, which
can be then used to calculate SLOs. The `/metrics/slis` endpoint provides the raw data necessary
to calculate an availability SLO for the respective Kubernetes component.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified January 07, 2025 at 8:06 PM PST: [Add description for Kubernetes Component SLI Metrics (6c83f61fa5)](https://github.com/kubernetes/website/commit/6c83f61fa5b0ac1e71feb574b98be72791b315c8)
## Related Pages

- [Switching from Polling to CRI Event-based Updates to Container Status](docs-tasks-administer-cluster-switch-to-evented-pleg.md)
- [Understand Pressure Stall Information (PSI) Metrics](docs-reference-instrumentation-understand-psi-metrics.md)
- [Adopting Sidecar Containers](docs-tutorials-configuration-pod-sidecar-containers.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)