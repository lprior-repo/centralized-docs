---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#12-standard
chunk_level: standard
chunk_type: prose
heading: ServiceSpec
token_count: 334
summary: * **sessionAffinityConfig** (SessionAffinityConfig) sessionAffinityConfig contains the configurations of session affinity. *SessionAffinityConfig represents the configurations of session affinity.* *...
---

* **sessionAffinityConfig** (SessionAffinityConfig)
sessionAffinityConfig contains the configurations of session affinity.
*SessionAffinityConfig represents the configurations of session affinity.*
* **sessionAffinityConfig.clientIP** (ClientIPConfig)
clientIP contains the configurations of Client IP based session affinity.
*ClientIPConfig represents the configurations of Client IP based session affinity.*
* **sessionAffinityConfig.clientIP.timeoutSeconds** (int32)
timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be &gt;0 &amp;&amp; &lt;=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).
* **allocateLoadBalancerNodePorts** (boolean)
allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer. Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts. If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.
* **trafficDistribution** (string)
TrafficDistribution offers a way to express preferences for how traffic is distributed to Service endpoints. Implementations can use this field as a hint, but are not required to guarantee strict adherence. If the field is not set, the implementation will apply its default routing strategy. If set to "PreferClose", implementations should prioritize endpoints that are in the same zone.