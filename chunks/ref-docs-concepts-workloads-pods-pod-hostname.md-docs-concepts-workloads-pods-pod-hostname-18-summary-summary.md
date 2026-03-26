---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#18-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's hostnameOverride
token_count: 112
summary: * The Pod's A and/or AAAA records in the cluster DNS server are still generated based on the `hostname` and `subdomain` fields. Note: If `hostnameOverride` is set, you cannot simultaneously set the...
---

* The Pod's A and/or AAAA records in the cluster DNS server are still generated based on the `hostname` and `subdomain` fields.
Note: If `hostnameOverride` is set, you cannot simultaneously set the `hostNetwork` and `setHostnameAsFQDN` fields.
The API server will explicitly reject any create request attempting this combination.
For details on behavior when `hostnameOverride` is set in combination with
other fields (hostname, subdomain, setHostnameAsFQDN, hostNetwork),
see the table in the