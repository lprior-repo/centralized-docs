---
id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget
title: kubectl create poddisruptionbudget
category: ref
tags: ["contents", "create", "kubectl", "poddisruptionbudget", "ref"]
---

## Table of Contents

* [kubectl create poddisruptionbudget](#kubectl-create-poddisruptionbudget)
  * [Synopsis](#synopsis)
  * [Examples](#examples)
* [and require at least one of them being available at any point in time](#and-require-at-least-one-of-them-being-available-at-any-point-in-time)
* [Create a pod disruption budget named my-pdb that will select all pods with the app=nginx label](#create-a-pod-disruption-budget-named-my-pdb-that-will-select-all-pods-with-the-appnginx-label)
* [and require at least half of the pods selected to be available at any point in time](#and-require-at-least-half-of-the-pods-selected-to-be-available-at-any-point-in-time)
  * [Options](#options)
  * [Parent Options Inherited](#parent-options-inherited)
  * [Feedback](#feedback)

---

# kubectl create poddisruptionbudget



 > 
 > **Context**: Create a pod disruption budget with the specified name



Create a pod disruption budget with the specified name

## Synopsis

Create a pod disruption budget with the specified name, selector, and desired minimum available pods.

````
`kubectl create poddisruptionbudget NAME --selector=SELECTOR --min-available=N [--dry-run=server|client|none]
`
````

## Examples

````
` # Create a pod disruption budget named my-pdb that will select all pods with the app=rails label
# and require at least one of them being available at any point in time
kubectl create poddisruptionbudget my-pdb --selector=app=rails --min-available=1
# Create a pod disruption budget named my-pdb that will select all pods with the app=nginx label
# and require at least half of the pods selected to be available at any point in time
kubectl create pdb my-pdb --selector=app=nginx --min-available=50%
`
````

## Options

\|–allow-missing-template-keysDefault: true|
\||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
\|
\|–dry-run string\[=“unchanged”\]Default: “none”\|
\||
Must be “none”, “server”, or “client”. If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
\|
\|–field-manager stringDefault: “kubectl-create”\|
\||
Name of the manager used to track field ownership.
\|
\|-h, –help|
\||
help for poddisruptionbudget
\|
\|–max-unavailable string|
\||
The maximum number or percentage of unavailable pods this budget requires.
\|
\|–min-available string|
\||
The minimum number or percentage of available pods this budget requires.
\|
\|-o, –output string|
\||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
\|
\|–save-config|
\||
If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.
\|
\|–selector string|
\||
A label selector to use for this budget. Only equality-based selector requirements are supported.
\|
\|–show-managed-fields|
\||
If true, keep the managedFields when printing objects in JSON or YAML format.
\|
\|–template string|
\||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates \[http://golang.org/pkg/text/template/#pkg-overview\].
\|
\|–validate string\[=“strict”\]Default: “strict”\|
\||
Must be one of: strict (or true), warn, ignore (or false). “true” or “strict” will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will fall back to less reliable client-side validation if not. “warn” will warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled on the API server, and behave as “ignore” otherwise. “false” or “ignore” will not perform any schema validation, silently dropping any unknown or duplicate fields.
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
