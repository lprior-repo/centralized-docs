---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#20-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 117
summary: Create the following Service that explicitly defines `PreferDualStack` in `.spec.ipFamilyPolicy`. Kubernetes will assign both IPv4 and IPv6 addresses (as this cluster has dual-stack enabled) and...
---

Create the following Service that explicitly defines `PreferDualStack` in `.spec.ipFamilyPolicy`.
Kubernetes will assign both IPv4 and IPv6 addresses (as this cluster has dual-stack enabled) and
select the `.spec.ClusterIP` from the list of `.spec.ClusterIPs` based on the address family of
the first element in the `.spec.ipFamilies` array.
[`service/networking/dual-stack-preferred-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-preferred-svc.yaml)