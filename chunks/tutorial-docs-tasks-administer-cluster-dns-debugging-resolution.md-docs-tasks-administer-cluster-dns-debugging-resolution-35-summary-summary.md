---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#35-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 57
summary: ### Does CoreDNS have sufficient permissions? CoreDNS must be able to list [service](/docs/concepts/services-networking/service/) and...
---

### Does CoreDNS have sufficient permissions?
CoreDNS must be able to list [service](/docs/concepts/services-networking/service/) and [endpointslice](/docs/concepts/services-networking/endpoint-slices/) related resources to properly resolve service names.
Sample error message: