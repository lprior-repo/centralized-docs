---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#9-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 953
summary: #### Response 200 ([Status](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/status/#Status)): OK 202...
---

#### Response
200 ([Status](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/status/#Status)): OK
202 ([Status](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/status/#Status)): Accepted
401: Unauthorized
#### Parameters
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [DeleteOptions](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/delete-options/#DeleteOptions)
* **continue** (*in query*): string
[continue](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#continue)
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldSelector** (*in query*): string
[fieldSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldSelector)
* **gracePeriodSeconds** (*in query*): integer
[gracePeriodSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#gracePeriodSeconds)
* **ignoreStoreReadErrorWithClusterBreakingPotential** (*in query*): boolean
[ignoreStoreReadErrorWithClusterBreakingPotential](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#ignoreStoreReadErrorWithClusterBreakingPotential)
* **labelSelector** (*in query*): string
[labelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#labelSelector)
* **limit** (*in query*): integer
[limit](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#limit)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **propagationPolicy** (*in query*): string
[propagationPolicy](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#propagationPolicy)
* **resourceVersion** (*in query*): string
[resourceVersion](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersion)
* **resourceVersionMatch** (*in query*): string
[resourceVersionMatch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersionMatch)
* **sendInitialEvents** (*in query*): boolean
[sendInitialEvents](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#sendInitialEvents)
* **timeoutSeconds** (*in query*): integer
[timeoutSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#timeoutSeconds)
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
Last modified October 03, 2025 at 8:48 PM PST: [Update resource API 1.34 with enum values (24e17cbfa6)](https://github.com/kubernetes/website/commit/24e17cbfa680ea9e379774eec976b9fb4eea6e77)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [ValidatingAdmissionPolicy](docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md)
- [ServiceAccount](docs-reference-kubernetes-api-authentication-resources-service-account-v1.md)
- [DeviceTaintRule v1alpha3](docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md)
- [CSINode](docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)