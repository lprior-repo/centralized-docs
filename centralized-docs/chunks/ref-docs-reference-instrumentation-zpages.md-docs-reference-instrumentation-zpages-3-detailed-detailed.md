---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 961
summary: ### flagz Enabled using the `ComponentFlagz` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ComponentFlagz), the `/flagz` endpoint shows you the command line arguments...
---

### flagz
Enabled using the `ComponentFlagz` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ComponentFlagz), the `/flagz` endpoint shows you the command line arguments that were used to start a component.
The `/flagz` plain text response from the API server looks something like:
```
`kube-apiserver flags
Warning: This endpoint is not meant to be machine parseable, has no formatting compatibility guarantees and is for debugging purposes only.
advertise-address=192.168.8.2
contention-profiling=false
enable-priority-and-fairness=true
profiling=true
authorization-mode=[Node,RBAC]
authorization-webhook-cache-authorized-ttl=5m0s
authorization-webhook-cache-unauthorized-ttl=30s
authorization-webhook-version=v1beta1
default-watch-cache-size=100
`
```
#### flagz (structured)
FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Starting with Kubernetes v1.35, the `/flagz` endpoint supports a structured,
versioned response format when requested with the appropriate `Accept` header.
Without an `Accept` header, the endpoint returns the plain text response format by default.
To request the structured response, use:
```
`Accept: application/json;v=v1alpha1;g=config.k8s.io;as=Flagz
`
```
#### Note:
If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`),
the server will respond with `406 Not Acceptable`.
Example structured response:
```
`{
"kind": "Flagz",
"apiVersion": "config.k8s.io/v1alpha1",
"metadata": {
"name": "kube-apiserver"
},
"flags": {
"advertise-address": "192.168.8.4",
"allow-privileged": "true",
"anonymous-auth": "true",
"authorization-mode": "[Node,RBAC]",
"enable-priority-and-fairness": "true",
"profiling": "true",
"default-watch-cache-size": "100"
}
}
`
```
The `config.k8s.io/v1alpha1` schema for the structured `/flagz` response is as follows:
```
`// Flagz is the config.k8s.io/v1alpha1 schema for the /flagz endpoint.
type Flagz struct {
// Kind is "Flagz".
Kind string `json:"kind"`
// APIVersion is the version of the object, e.g., "config.k8s.io/v1alpha1".
APIVersion string `json:"apiVersion"`
// Standard object's metadata.
// +optional
Metadata metav1.ObjectMeta `json:"metadata,omitempty"`
// Flags contains the command-line flags and their values.
// The keys are the flag names and the values are the flag values,
// possibly with confidential values redacted.
// +optional
Flags map[string]string `json:"flags,omitempty"`
}
`
```
#### Note:
The structured responses for both `/statusz` and `/flagz` are alpha features in v1.35
and are subject to change in future releases.
They are intended to provide machine-parseable output for debugging and introspection tools.
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
Last modified October 15, 2025 at 9:38 AM PST: [zpages structured response (63cc27b920)](https://github.com/kubernetes/website/commit/63cc27b9209a578df42e2cbaf03fa49b8db8f78a)
## Related Pages

- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)