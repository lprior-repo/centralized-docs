---
url: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_label/
title: Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing value
word_count: 1213
filtered: true
elements_removed: 0
density_score: 0.85
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing value](#update-pod-foo-with-the-label-status-and-the-value-unhealthy-overwriting-any-existing-value)
- [Update all pods in the namespace](#update-all-pods-in-the-namespace)
- [Update a pod identified by the type and name in "pod.json"](#update-a-pod-identified-by-the-type-and-name-in-podjson)
- [Update pod 'foo' only if the resource is unchanged from version 1](#update-pod-foo-only-if-the-resource-is-unchanged-from-version-1)
- [Update pod 'foo' by removing a label named 'bar' if it exists](#update-pod-foo-by-removing-a-label-named-bar-if-it-exists)
- [Does not require the --overwrite flag](#does-not-require-the---overwrite-flag)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Update the labels on a resource.
* A label key and value must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters each.
* Optionally, the key can begin with a DNS subdomain prefix and a single '/', like example.com/my-app.
* If --overwrite is true, then existing labels can be overwritten, otherwise attempting to overwrite a label will result in an error.
* If --resource-version is specified, then updates will use this resource version, otherwise the existing resource-version will be used.
```
`kubectl label [--overwrite] (-f FILENAME | TYPE NAME) KEY\_1=VAL\_1 ... KEY\_N=VAL\_N [--resource-version=version]
`
```
## Examples
```
` # Update pod 'foo' with the label 'unhealthy' and the value 'true'
kubectl label pods foo unhealthy=true
# Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing value
kubectl label --overwrite pods foo status=unhealthy
# Update all pods in the namespace
kubectl label pods --all status=unhealthy
# Update a pod identified by the type and name in "pod.json"
kubectl label -f pod.json status=unhealthy
# Update pod 'foo' only if the resource is unchanged from version 1
kubectl label pods foo status=unhealthy --resource-version=1
# Update pod 'foo' by removing a label named 'bar' if it exists
# Does not require the --overwrite flag
kubectl label pods foo bar-
`
```
## Options
|--all|
||
Select all resources, in the namespace of the specified resource types
|
|-A, --all-namespaces|
||
If true, check the specified action in all namespaces.
|
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--field-manager stringDefault: "kubectl-label"|
||
Name of the manager used to track field ownership.
|
|--field-selector string|
||
Selector (field query) to filter on, supports '=', '==', and '!='.(e.g. --field-selector key1=value1,key2=value2). The server only supports a limited number of field queries per type.
|
|-f, --filename strings|
||
Filename, directory, or URL to files identifying the resource to update the labels
|
|-h, --help|
||
help for label
|
|-k, --kustomize string|
||
Process the kustomization directory. This flag can't be used together with -f or -R.
|
|--list|
||
If true, display the labels for a given resource.
|
|--local|
||
If true, label will NOT contact api-server but run locally.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--overwrite|
||
If true, allow labels to be overwritten, otherwise reject label updates that overwrite existing labels.
|
|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|--resource-version string|
||
If non-empty, the labels update will only succeed if this is the current resource-version for the object. Only valid when specifying a single resource.
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|
## Parent Options Inherited
|--as string|
||
Username to impersonate for the operation. User could be a regular user or a service account in a namespace.
|
|--as-group strings|
||
Group to impersonate for the operation, this flag can be repeated to specify multiple groups.
|
|--as-uid string|
||
UID to impersonate for the operation.
|
|--as-user-extra strings|
||
User extras to impersonate for the operation, this flag can be repeated to specify multiple values for the same key.
|
|--cache-dir stringDefault: "$HOME/.kube/cache"|
||
Default cache directory
|
|--certificate-authority string|
||
Path to a cert file for the certificate authority
|
|--client-certificate string|
||
Path to a client certificate file for TLS
|
|--client-key string|
||
Path to a client key file for TLS
|
|--cluster string|
||
The name of the kubeconfig cluster to use
|
|--context string|
||
The name of the kubeconfig context to use
|
|--disable-compression|
||
If true, opt-out of response compression for all requests to the server
|
|--insecure-skip-tls-verify|
||
If true, the server's certificate will not be checked for validity. This will make your HTTPS connections insecure
|
|--kubeconfig string|
||
Path to the kubeconfig file to use for CLI requests.
|
|--kuberc string|
||
Path to the kuberc file to use for preferences. This can be disabled by exporting KUBECTL\_KUBERC=false feature gate or turning off the feature KUBERC=off.
|
|--match-server-version|
||
Require server version to match client version
|
|-n, --namespace string|
||
If present, the namespace scope for this CLI request
|
|--password string|
||
Password for basic authentication to the API server
|
|--profile stringDefault: "none"|
||
Name of profile to capture. One of (none|cpu|heap|goroutine|threadcreate|block|mutex|trace)
|
|--profile-output stringDefault: "profile.pprof"|
||
Name of the file to write the profile to
|
|--request-timeout stringDefault: "0"|
||
The length of time to wait before giving up on a single server request. Non-zero values should contain a corresponding time unit (e.g. 1s, 2m, 3h). A value of zero means don't timeout requests.
|
|-s, --server string|
||
The address and port of the Kubernetes API server
|
|--storage-driver-buffer-duration durationDefault: 1m0s|
||
Writes in the storage driver will be buffered for this duration, and committed to the non memory backends as a single transaction
|
|--storage-driver-db stringDefault: "cadvisor"|
||
database name
|
|--storage-driver-host stringDefault: "localhost:8086"|
||
database host:port
|
|--storage-driver-password stringDefault: "root"|
||
database password
|
|--storage-driver-secure|
||
use secure connection with database
|
|--storage-driver-table stringDefault: "stats"|
||
table name
|
|--storage-driver-user stringDefault: "root"|
||
database username
|
|--tls-server-name string|
||
Server name to use for server certificate validation. If it is not provided, the hostname used to contact the server is used
|
|--token string|
||
Bearer token for authentication to the API server
|
|--user string|
||
The name of the kubeconfig user to use
|
|--username string|
||
Username for basic authentication to the API server
|
|--version version[=true]|
||
--version, --version=raw prints version information and quits; --version=vX.Y.Z... sets the reported version
|
|--warnings-as-errors|
||
Treat warnings received from the server as errors and exit with a non-zero exit code
|
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

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)
