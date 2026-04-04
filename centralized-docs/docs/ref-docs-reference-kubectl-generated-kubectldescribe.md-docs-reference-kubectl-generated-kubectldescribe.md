---
id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
title: kubectl describe
category: ref
tags: ["contents", "describe", "kubectl", "ref", "synopsis"]
---

## Table of Contents

* [kubectl describe](#kubectl-describe)
  * [Synopsis](#synopsis)
  * [Examples](#examples)
* [Describe a pod identified by type and name in “pod.json”](#describe-a-pod-identified-by-type-and-name-in-podjson)
* [Describe pods by label name=myLabel](#describe-pods-by-label-namemylabel)
* [Describe all pods managed by the ‘frontend’ replication controller](#describe-all-pods-managed-by-the-frontend-replication-controller)
* [(rc-created pods get the name of the rc as a prefix in the pod name)](#rc-created-pods-get-the-name-of-the-rc-as-a-prefix-in-the-pod-name)
  * [Options](#options)
  * [Parent Options Inherited](#parent-options-inherited)
  * [Feedback](#feedback)

---

# kubectl describe



 > 
 > **Context**: Show details of a specific resource or group of resources



Show details of a specific resource or group of resources

## Synopsis

Show details of a specific resource or group of resources.
Print a detailed description of the selected resources, including related resources such as events or controllers. You may select a single object by name, all objects of that type, provide a name prefix, or label selector. For example:

````
` $ kubectl describe TYPE NAME\_PREFIX
`
````

will first check for an exact match on TYPE and NAME\_PREFIX. If no such resource exists, it will output details for every resource that has a name prefixed with NAME\_PREFIX.
Use “kubectl api-resources” for a complete list of supported resources.

````
`kubectl describe (-f FILENAME | TYPE [NAME\_PREFIX | -l label] | TYPE/NAME)
`
````

## Examples

````
` # Describe a node
kubectl describe nodes kubernetes-node-emt8.c.myproject.internal
# Describe a pod identified by type and name in "pod.json"
kubectl describe -f pod.json
# Describe pods by label name=myLabel
kubectl describe pods -l name=myLabel
# Describe all pods managed by the 'frontend' replication controller
# (rc-created pods get the name of the rc as a prefix in the pod name)
kubectl describe pods frontend
`
````

## Options

\|-A, –all-namespaces|
\||
If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even if specified with –namespace.
\|
\|–chunk-size intDefault: 500|
\||
Return large lists in chunks rather than all at once. Pass 0 to disable.
\|
\|-f, –filename strings|
\||
Filename, directory, or URL to files containing the resource to describe
\|
\|-h, –help|
\||
help for describe
\|
\|-k, –kustomize string|
\||
Process the kustomization directory. This flag can’t be used together with -f or -R.
\|
\|-R, –recursive|
\||
Process the directory used in -f, –filename recursively. Useful when you want to manage related manifests organized within the same directory.
\|
\|-l, –selector string|
\||
Selector (label query) to filter on, supports ‘=’, ‘==’, ‘!=’, ‘in’, ‘notin’.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
\|
\|–show-eventsDefault: true|
\||
If true, display events related to the described object.
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
