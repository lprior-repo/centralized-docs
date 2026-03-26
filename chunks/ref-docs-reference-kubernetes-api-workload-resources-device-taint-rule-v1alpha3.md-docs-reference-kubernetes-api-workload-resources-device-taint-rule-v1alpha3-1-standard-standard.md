---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#1-standard
chunk_level: standard
chunk_type: prose
heading: DeviceTaintRule
token_count: 262
summary: # DeviceTaintRule v1alpha3 DeviceTaintRule adds one taint to all devices which match the selector. `apiVersion: resource.k8s.io/v1alpha3` `import \"k8s.io/api/resource/v1alpha3\"` ## DeviceTaintRule...
---

# DeviceTaintRule v1alpha3
DeviceTaintRule adds one taint to all devices which match the selector.
`apiVersion: resource.k8s.io/v1alpha3`
`import "k8s.io/api/resource/v1alpha3"`
## DeviceTaintRule
DeviceTaintRule adds one taint to all devices which match the selector. This has the same effect as if the taint was specified directly in the ResourceSlice by the DRA driver.
* **apiVersion**: resource.k8s.io/v1alpha3
* **kind**: DeviceTaintRule
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object metadata
* **spec** ([DeviceTaintRuleSpec](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRuleSpec)), required
Spec specifies the selector and one taint.
Changing the spec automatically increments the metadata.generation number.
* **status** ([DeviceTaintRuleStatus](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRuleStatus))
Status provides information about what was requested in the spec.