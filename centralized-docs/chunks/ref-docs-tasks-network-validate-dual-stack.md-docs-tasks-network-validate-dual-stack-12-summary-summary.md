---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#12-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 112
summary: ## Validate Services Create the following Service that does not explicitly define `.spec.ipFamilyPolicy`. Kubernetes will assign a cluster IP for the Service from the first configured...
---

## Validate Services
Create the following Service that does not explicitly define `.spec.ipFamilyPolicy`.
Kubernetes will assign a cluster IP for the Service from the first configured
`service-cluster-ip-range` and set the `.spec.ipFamilyPolicy` to `SingleStack`.
[`service/networking/dual-stack-default-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-default-svc.yaml)![](/images/copycode.svg "Copy service/networking/dual-stack-default-svc.yaml to clipboard")