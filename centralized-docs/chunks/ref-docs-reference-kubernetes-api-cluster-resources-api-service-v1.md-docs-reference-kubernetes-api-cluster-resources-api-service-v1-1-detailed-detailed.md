---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#1-detailed
chunk_level: detailed
chunk_type: prose
heading: APIServiceSpec
token_count: 981
summary: # APIService APIService represents a server for a particular GroupVersion. `apiVersion: apiregistration.k8s.io/v1` `import \"k8s.io/kube-aggregator/pkg/apis/apiregistration/v1\"` ## APIService...
---

# APIService
APIService represents a server for a particular GroupVersion.
`apiVersion: apiregistration.k8s.io/v1`
`import "k8s.io/kube-aggregator/pkg/apis/apiregistration/v1"`
## APIService
APIService represents a server for a particular GroupVersion. Name must be "version.group".
* **apiVersion**: apiregistration.k8s.io/v1
* **kind**: APIService
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **spec** ([APIServiceSpec](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIServiceSpec))
Spec contains information for locating and communicating with a server
* **status** ([APIServiceStatus](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIServiceStatus))
Status contains derived information about an API server
## APIServiceSpec
APIServiceSpec contains information for locating and communicating with a server. Only https is supported, though you are able to disable certificate verification.
* **groupPriorityMinimum** (int32), required
GroupPriorityMinimum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones. Note that other versions of this group might specify even higher GroupPriorityMinimum values such that the whole group gets a higher priority. The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object. (v1.bar before v1.foo) We'd recommend something like: \*.k8s.io (except extensions) at 18000 and PaaSes (OpenShift, Deis) are recommended to be in the 2000s
* **versionPriority** (int32), required
VersionPriority controls the ordering of this API version inside of its group. Must be greater than zero. The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10). Since it's inside of a group, the number can be small, probably in the 10s. In case of equal version priorities, the version string will be used to compute the order inside a group. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA &gt; beta &gt; alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
* **caBundle** ([]byte)
*Atomic: will be replaced during a merge*
CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate. If unspecified, system trust roots on the apiserver are used.
* **group** (string)
Group is the API group name this server hosts
* **insecureSkipTLSVerify** (boolean)
InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged. You should use the CABundle instead.
* **service** (ServiceReference)
Service is a reference to the service for this API server. It must communicate on port 443. If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled.
*ServiceReference holds a reference to Service.legacy.k8s.io*
* **service.name** (string)
Name is the name of the service
* **service.namespace** (string)
Namespace is the namespace of the service
* **service.port** (int32)
If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
* **version** (string)
Version is the API version this server hosts. For example, "v1"