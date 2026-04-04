---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#27-standard
chunk_level: standard
chunk_type: table
heading: Default roles and role bindings
token_count: 431
summary: ### Other component roles |Default ClusterRole|Default ClusterRoleBinding|Description| |**system:auth-delegator**|None|Allows delegated authentication and authorization checks. This is commonly used...
---

### Other component roles
|Default ClusterRole|Default ClusterRoleBinding|Description|
|**system:auth-delegator**|None|Allows delegated authentication and authorization checks.
This is commonly used by add-on API servers for unified authentication and authorization.|
|**system:heapster**|None|Role for the [Heapster](https://github.com/kubernetes/heapster) component (deprecated).|
|**system:kube-aggregator**|None|Role for the [kube-aggregator](https://github.com/kubernetes/kube-aggregator) component.|
|**system:kube-dns**|**kube-dns** service account in the **kube-system** namespace|Role for the [kube-dns](/docs/concepts/services-networking/dns-pod-service/) component.|
|**system:kubelet-api-admin**|None|Allows full access to the kubelet API.|
|**system:node-bootstrapper**|None|Allows access to the resources required to perform
[kubelet TLS bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/).|
|**system:node-problem-detector**|None|Role for the [node-problem-detector](https://github.com/kubernetes/node-problem-detector) component.|
|**system:persistent-volume-provisioner**|None|Allows access to the resources required by most [dynamic volume provisioners](/docs/concepts/storage/persistent-volumes/#dynamic).|
|**system:monitoring**|**system:monitoring** group|Allows read access to control-plane monitoring endpoints (i.e. [kube-apiserver](/docs/concepts/architecture/#kube-apiserver) liveness and readiness endpoints (/healthz, /livez, /readyz), the individual health-check endpoints (/healthz/\*, /livez/\*, /readyz/\*), /metrics), and causes the kube-apiserver to respect the traceparent header provided with requests for tracing. Note that individual health check endpoints and the metric endpoint may expose sensitive information.|