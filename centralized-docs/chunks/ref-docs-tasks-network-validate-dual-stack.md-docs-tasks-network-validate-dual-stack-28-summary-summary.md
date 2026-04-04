---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#28-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 112
summary: If the cloud provider supports the provisioning of IPv6 enabled external load balancers, create the following Service with `PreferDualStack` in `.spec.ipFamilyPolicy`, `IPv6` as the first element of...
---

If the cloud provider supports the provisioning of IPv6 enabled external load balancers,
create the following Service with `PreferDualStack` in `.spec.ipFamilyPolicy`, `IPv6` as
the first element of the `.spec.ipFamilies` array and the `type` field set to `LoadBalancer`.
[`service/networking/dual-stack-prefer-ipv6-lb-svc.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/dual-stack-prefer-ipv6-lb-svc.yaml)