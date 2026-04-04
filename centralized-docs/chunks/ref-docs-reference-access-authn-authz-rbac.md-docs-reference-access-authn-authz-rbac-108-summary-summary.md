---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#108-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 123
summary: .| |**system:monitoring**|**system:monitoring** group|Allows read access to control-plane monitoring endpoints (i.e. [kube-apiserver](/docs/concepts/architecture/#kube-apiserver) liveness and...
---

.|
|**system:monitoring**|**system:monitoring** group|Allows read access to control-plane monitoring endpoints (i.e. [kube-apiserver](/docs/concepts/architecture/#kube-apiserver) liveness and readiness endpoints (/healthz, /livez, /readyz), the individual health-check endpoints (/healthz/\*, /livez/\*, /readyz/\*), /metrics), and causes the kube-apiserver to respect the traceparent header provided with requests for tracing. Note that individual health check endpoints and the metric endpoint may expose sensitive information.|