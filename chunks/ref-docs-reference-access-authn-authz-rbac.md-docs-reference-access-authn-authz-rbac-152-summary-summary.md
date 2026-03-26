---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#152-summary
chunk_level: summary
chunk_type: prose
heading: Write access for EndpointSlices
token_count: 123
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: annotations: kubernetes.io/description: |- Add endpoints write permissions to the edit and admin roles. This was removed by...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
annotations:
kubernetes.io/description: |-
Add endpoints write permissions to the edit and admin roles. This was
removed by default in 1.22 because of CVE-2021-25740. See
https://issue.k8s.io/103675. This can allow writers to direct LoadBalancer
or Ingress implementations to expose backend IPs that would not otherwise
be accessible, and can circumvent network policies or security controls
intended to prevent/isolate access to those backends.