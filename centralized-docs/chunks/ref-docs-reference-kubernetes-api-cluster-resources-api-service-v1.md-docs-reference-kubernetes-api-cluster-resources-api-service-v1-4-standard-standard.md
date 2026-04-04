---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: APIServiceStatus
token_count: 450
summary: * **group** (string) Group is the API group name this server hosts * **insecureSkipTLSVerify** (boolean) InsecureSkipTLSVerify disables TLS certificate verification when communicating with this...
---

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
## APIServiceStatus
APIServiceStatus contains derived information about an API server
* **conditions** ([]APIServiceCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Current service state of apiService.
*APIServiceCondition describes the state of an APIService at a particular point*
* **conditions.status** (string), required
Status is the status of the condition. Can be True, False, Unknown.
* **conditions.type** (string), required
Type is the type of the condition.
* **conditions.lastTransitionTime** (Time)
Last time the condition transitioned from one status to another.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string)
Human-readable message indicating details about last transition.
* **conditions.reason** (string)
Unique, one-word, CamelCase reason for the condition's last transition.