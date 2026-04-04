---
id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
title: Kubernetes z-pages
category: ref
tags: ["contents", "kubernetes", "ref", "table", "z-pages"]
---

## Table of Contents

* [Kubernetes z-pages](#kubernetes-z-pages)
  * [z-pages](#z-pages)
    * [statusz (structured)](#statusz-structured)
    * [Note:](#note)
    * [flagz](#flagz)
      * [flagz (structured)](#flagz-structured)
      * [Note:](#note)
      * [Note:](#note)
  * [Feedback](#feedback)

---

# Kubernetes z-pages



 > 
 > **Context**: Provides runtime diagnostics for Kubernetes components, offering insights into component runtime status and configuration flags. FEATURE STATE: Kubern



Provides runtime diagnostics for Kubernetes components, offering insights into component runtime status and configuration flags.
FEATURE STATE:
`Kubernetes v1.32 [alpha]`
Kubernetes core components can expose a suite of *z-endpoints* to make it easier for users
to debug their cluster and its components. These endpoints are strictly to be used for human
inspection to gain real time debugging information of a component binary.
Avoid automated scraping of data returned by these endpoints; in Kubernetes 1.35
these are an **alpha** feature and the response format may change in future releases.

## z-pages

Kubernetes v1.35 allows you to enable *z-pages* to help you troubleshoot
problems with its core control plane components. These special debugging endpoints provide internal
information about running components. For Kubernetes 1.35, components
serve the following endpoints (when enabled):

* [z-pages](#z-pages)
* [statusz](#statusz)
* [statusz (structured)](#statusz-structured)
* [flagz](#flagz)
* [flagz (structured)](#flagz-structured)\### statusz
  Enabled using the `ComponentStatusz` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ComponentStatusz),
  the `/statusz` endpoint displays high level information about the component such as its Kubernetes version, emulation version, start time and more.
  The `/statusz` plain text response from the API server is similar to:

````
`kube-apiserver statusz
Warning: This endpoint is not meant to be machine parseable, has no formatting compatibility guarantees and is for debugging purposes only.
Started: Wed Oct 16 21:03:43 UTC 2024
Up: 0 hr 00 min 16 sec
Go version: go1.23.2
Binary version: 1.32.0-alpha.0.1484&amp;#43;5eeac4f21a491b-dirty
Emulation version: 1.32.0-alpha.0.1484
Paths: /healthz /livez /metrics /readyz /statusz /version
`
````

### statusz (structured)

FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Starting with Kubernetes v1.35, the `/statusz` endpoint supports a structured,
versioned response format when requested with the appropriate `Accept` header.
Without an `Accept` header, the endpoint returns the plain text response format by default.
To request the structured response, use:

````
`Accept: application/json;v=v1alpha1;g=config.k8s.io;as=Statusz
`
````

#### Note:

If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`),
the server will respond with `406 Not Acceptable`.
Example structured response:

````
`{
"kind": "Statusz",
"apiVersion": "config.k8s.io/v1alpha1",
"metadata": {
"name": "kube-apiserver"
},
"startTime": "2025-10-29T00:30:01Z",
"uptimeSeconds": 856,
"goVersion": "go1.23.2",
"binaryVersion": "1.35.0",
"emulationVersion": "1.35",
"paths": [
"/healthz",
"/livez",
"/metrics",
"/readyz",
"/statusz",
"/version"
]
}
`
````

The `config.k8s.io/v1alpha1` schema for the structured `/statusz` response is as follows:

````
`// Statusz is the config.k8s.io/v1alpha1 schema for the /statusz endpoint.
type Statusz struct {
// Kind is "Statusz".
Kind string `json:"kind"`
// APIVersion is the version of the object, e.g., "config.k8s.io/v1alpha1".
APIVersion string `json:"apiVersion"`
// Standard object's metadata.
// +optional
Metadata metav1.ObjectMeta `json:"metadata,omitempty"`
// StartTime is the time the component process was initiated.
StartTime metav1.Time `json:"startTime"`
// UptimeSeconds is the duration in seconds for which the component has been running continuously.
UptimeSeconds int64 `json:"uptimeSeconds"`
// GoVersion is the version of the Go programming language used to build the binary.
// The format is not guaranteed to be consistent across different Go builds.
// +optional
GoVersion string `json:"goVersion,omitempty"`
// BinaryVersion is the version of the component's binary.
// The format is not guaranteed to be semantic versioning and may be an arbitrary string.
BinaryVersion string `json:"binaryVersion"`
// EmulationVersion is the Kubernetes API version which this component is emulating.
// if present, formatted as "&lt;major&gt;.&lt;minor&gt;"
// +optional
EmulationVersion string `json:"emulationVersion,omitempty"`
// MinimumCompatibilityVersion is the minimum Kubernetes API version with which the component is designed to work.
// if present, formatted as "&lt;major&gt;.&lt;minor&gt;"
// +optional
MinimumCompatibilityVersion string `json:"minimumCompatibilityVersion,omitempty"`
// Paths contains relative URLs to other essential read-only endpoints for debugging and troubleshooting.
// +optional
Paths []string `json:"paths,omitempty"`
}
`
````

### flagz

Enabled using the `ComponentFlagz` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ComponentFlagz), the `/flagz` endpoint shows you the command line arguments that were used to start a component.
The `/flagz` plain text response from the API server looks something like:

````
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
````

#### flagz (structured)

FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Starting with Kubernetes v1.35, the `/flagz` endpoint supports a structured,
versioned response format when requested with the appropriate `Accept` header.
Without an `Accept` header, the endpoint returns the plain text response format by default.
To request the structured response, use:

````
`Accept: application/json;v=v1alpha1;g=config.k8s.io;as=Flagz
`
````

#### Note:

If you request `application/json` without specifying all required parameters (`g`, `v`, and `as`),
the server will respond with `406 Not Acceptable`.
Example structured response:

````
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
````

The `config.k8s.io/v1alpha1` schema for the structured `/flagz` response is as follows:

````
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
````

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

* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
* [Using RBAC Authorization](./ref-docs-reference-access-authn-authz-rbac.md-docs-reference-access-authn-authz-rbac.md)
## See Also

- [Documentation Index](./COMPASS.md)
