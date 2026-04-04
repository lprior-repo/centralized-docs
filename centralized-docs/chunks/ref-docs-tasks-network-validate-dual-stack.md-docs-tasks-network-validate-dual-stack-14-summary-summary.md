---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#14-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 75
summary: Use `kubectl` to view the YAML for the Service. ``` `kubectl get svc my-service -o yaml ` ``` The Service has `.spec.ipFamilyPolicy` set to `SingleStack` and `.spec.clusterIP` set to an IPv4 address...
---

Use `kubectl` to view the YAML for the Service.
```
`kubectl get svc my-service -o yaml
`
```
The Service has `.spec.ipFamilyPolicy` set to `SingleStack` and `.spec.clusterIP` set
to an IPv4 address from the first configured range set via `--service-cluster-ip-range`
flag on kube-controller-manager.