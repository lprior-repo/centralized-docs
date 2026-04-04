---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#1-standard
chunk_level: standard
chunk_type: prose
heading: APIService
token_count: 257
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