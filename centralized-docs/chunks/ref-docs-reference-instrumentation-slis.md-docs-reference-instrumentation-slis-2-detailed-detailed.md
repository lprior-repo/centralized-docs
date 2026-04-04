---
doc_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 369
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

- [Certificates and Certificate Signing Requests](docs-reference-access-authn-authz-certificate-signing-requests.md)
- [Use an Image Volume With a Pod](docs-tasks-configure-pod-container-image-volumes.md)
- [Volumes](docs-concepts-storage-volumes.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)