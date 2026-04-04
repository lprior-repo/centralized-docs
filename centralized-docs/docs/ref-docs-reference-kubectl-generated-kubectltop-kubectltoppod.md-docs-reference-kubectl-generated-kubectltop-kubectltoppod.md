---
id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod
title: kubectl top pod
category: ref
tags: ["contents", "kubectl", "ref", "synopsis", "table"]
---

## Table of Contents

* [kubectl top pod](#kubectl-top-pod)
  * [Synopsis](#synopsis)
  * [Examples](#examples)
* [Show metrics for all pods in the given namespace](#show-metrics-for-all-pods-in-the-given-namespace)
* [Show metrics for a given pod and its containers](#show-metrics-for-a-given-pod-and-its-containers)
* [Show metrics for the pods defined by label name=myLabel](#show-metrics-for-the-pods-defined-by-label-namemylabel)
  * [Options](#options)
  * [Parent Options Inherited](#parent-options-inherited)
  * [Feedback](#feedback)

---

# kubectl top pod



 > 
 > **Context**: Display resource (CPU/memory) usage of pods



Display resource (CPU/memory) usage of pods

## Synopsis

Display resource (CPU/memory) usage of pods.
The ‘top pod’ command allows you to see the resource consumption of pods.
Due to the metrics pipeline delay, they may be unavailable for a few minutes since pod creation.

````
`kubectl top pod [NAME | -l label]
`
````

## Examples

````
` # Show metrics for all pods in the default namespace
kubectl top pod
# Show metrics for all pods in the given namespace
kubectl top pod --namespace=NAMESPACE
# Show metrics for a given pod and its containers
kubectl top pod POD\_NAME --containers
# Show metrics for the pods defined by label name=myLabel
kubectl top pod -l name=myLabel
`
````

## Options

\|-A, –all-namespaces|
\||
If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even if specified with –namespace.
\|
\|–containers|
\||
If present, print usage of containers within a pod.
\|
\|–field-selector string|
\||
Selector (field query) to filter on, supports ‘=’, ‘==’, and ‘!=’.(e.g. –field-selector key1=value1,key2=value2). The server only supports a limited number of field queries per type.
\|
\|-h, –help|
\||
help for pod
\|
\|–no-headers|
\||
If present, print output without headers.
\|
\|-l, –selector string|
\||
Selector (label query) to filter on, supports ‘=’, ‘==’, ‘!=’, ‘in’, ‘notin’.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
\|
\|–show-swap|
\||
Print pod resources related to swap memory.
\|
\|–sort-by string|
\||
If non-empty, sort pods list using specified field. The field can be either ‘cpu’ or ‘memory’.
\|
\|–sum|
\||
Print the sum of the resource usage
\|
\|–use-protocol-buffersDefault: true|
\||
Enables using protocol-buffers to access Metrics API.
\|

## Parent Options Inherited

\|–as string|
\||
Username to impersonate for the operation. User could be a regular user or a service account in a namespace.
\|
\|–as-group strings|
\||
Group to impersonate for the operation, this flag can be repeated to specify multiple groups.
\|
\|–as-uid string|
\||
UID to impersonate for the operation.
\|
\|–as-user-extra strings|
\||
User extras to impersonate for the operation, this flag can be repeated to specify multiple values for the same key.
\|
\|–cache-dir stringDefault: “$HOME/.kube/cache”\|
\||
Default cache directory
\|
\|–certificate-authority string|
\||
Path to a cert file for the certificate authority
\|
\|–client-certificate string|
\||
Path to a client certificate file for TLS
\|
\|–client-key string|
\||
Path to a client key file for TLS
\|
\|–cluster string|
\||
The name of the kubeconfig cluster to use
\|
\|–context string|
\||
The name of the kubeconfig context to use
\|
\|–disable-compression|
\||
If true, opt-out of response compression for all requests to the server
\|
\|–insecure-skip-tls-verify|
\||
If true, the server’s certificate will not be checked for validity. This will make your HTTPS connections insecure
\|
\|–kubeconfig string|
\||
Path to the kubeconfig file to use for CLI requests.
\|
\|–kuberc string|
\||
Path to the kuberc file to use for preferences. This can be disabled by exporting KUBECTL\_KUBERC=false feature gate or turning off the feature KUBERC=off.
\|
\|–match-server-version|
\||
Require server version to match client version
\|
\|-n, –namespace string|
\||
If present, the namespace scope for this CLI request
\|
\|–password string|
\||
Password for basic authentication to the API server
\|
\|–profile stringDefault: “none”\|
\||
Name of profile to capture. One of (none|cpu|heap|goroutine|threadcreate|block|mutex|trace)
\|
\|–profile-output stringDefault: “profile.pprof”\|
\||
Name of the file to write the profile to
\|
\|–request-timeout stringDefault: “0”\|
\||
The length of time to wait before giving up on a single server request. Non-zero values should contain a corresponding time unit (e.g. 1s, 2m, 3h). A value of zero means don’t timeout requests.
\|
\|-s, –server string|
\||
The address and port of the Kubernetes API server
\|
\|–storage-driver-buffer-duration durationDefault: 1m0s|
\||
Writes in the storage driver will be buffered for this duration, and committed to the non memory backends as a single transaction
\|
\|–storage-driver-db stringDefault: “cadvisor”\|
\||
database name
\|
\|–storage-driver-host stringDefault: “localhost:8086”\|
\||
database host:port
\|
\|–storage-driver-password stringDefault: “root”\|
\||
database password
\|
\|–storage-driver-secure|
\||
use secure connection with database
\|
\|–storage-driver-table stringDefault: “stats”\|
\||
table name
\|
\|–storage-driver-user stringDefault: “root”\|
\||
database username
\|
\|–tls-server-name string|
\||
Server name to use for server certificate validation. If it is not provided, the hostname used to contact the server is used
\|
\|–token string|
\||
Bearer token for authentication to the API server
\|
\|–user string|
\||
The name of the kubeconfig user to use
\|
\|–username string|
\||
Username for basic authentication to the API server
\|
\|–version version\[=true\]\|
\||
–version, –version=raw prints version information and quits; –version=vX.Y.Z… sets the reported version
\|
\|–warnings-as-errors|
\||
Treat warnings received from the server as errors and exit with a non-zero exit code
\|

## Feedback

Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified March 22, 2026 at 3:35 PM PST: [Switch to Hugo page discovery and add description field to kubectl command front matter (83100f9a0d)](https://github.com/kubernetes/website/commit/83100f9a0db1c9a1954de0d9552c4733e73029ae)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.

## Related Pages

* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
* [Using RBAC Authorization](./ref-docs-reference-access-authn-authz-rbac.md-docs-reference-access-authn-authz-rbac.md)
## See Also

- [Documentation Index](./COMPASS.md)
