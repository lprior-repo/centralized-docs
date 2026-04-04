---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#13-detailed
chunk_level: detailed
chunk_type: table
heading: Default roles and role bindings
token_count: 891
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
### Roles for built-in controllers
The Kubernetes [controller manager](/docs/reference/command-line-tools-reference/kube-controller-manager/) runs
[controllers](/docs/concepts/architecture/controller/) that are built in to the Kubernetes
control plane.
When invoked with `--use-service-account-credentials`, kube-controller-manager starts each controller
using a separate service account.
Corresponding roles exist for each built-in controller, prefixed with `system:controller:`.
If the controller manager is not started with `--use-service-account-credentials`, it runs all control loops
using its own credential, which must be granted all the relevant roles.
These roles include:
* `system:controller:attachdetach-controller`
* `system:controller:certificate-controller`
* `system:controller:clusterrole-aggregation-controller`
* `system:controller:cronjob-controller`
* `system:controller:daemon-set-controller`
* `system:controller:deployment-controller`
* `system:controller:disruption-controller`
* `system:controller:endpoint-controller`
* `system:controller:expand-controller`
* `system:controller:generic-garbage-collector`
* `system:controller:horizontal-pod-autoscaler`
* `system:controller:job-controller`
* `system:controller:namespace-controller`
* `system:controller:node-controller`
* `system:controller:persistent-volume-binder`
* `system:controller:pod-garbage-collector`
* `system:controller:pv-protection-controller`
* `system:controller:pvc-protection-controller`
* `system:controller:replicaset-controller`
* `system:controller:replication-controller`
* `system:controller:resourcequota-controller`
* `system:controller:root-ca-cert-publisher`
* `system:controller:route-controller`
* `system:controller:service-account-controller`
* `system:controller:service-controller`
* `system:controller:statefulset-controller`
* `system:controller:ttl-controller`## Privilege escalation prevention and bootstrapping
The RBAC API prevents users from escalating privileges by editing roles or role bindings.
Because this is enforced at the API level, it applies even when the RBAC authorizer is not in use.