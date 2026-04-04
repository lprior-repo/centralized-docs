---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#14-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 124
summary: * **ports.targetPort** (IntOrString) Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA\_SVC\_NAME. If this is a...
---

* **ports.targetPort** (IntOrString)
Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA\_SVC\_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: