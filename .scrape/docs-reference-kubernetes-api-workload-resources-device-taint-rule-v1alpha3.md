---
url: https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/
title: DeviceTaintRule v1alpha3
word_count: 1561
filtered: true
elements_removed: 0
density_score: 0.94
---

## Table of Contents

- [DeviceTaintRule v1alpha3](#devicetaintrule-v1alpha3)
  - [DeviceTaintRule](#devicetaintrule)
  - [DeviceTaintRuleSpec](#devicetaintrulespec)
  - [DeviceTaintRuleStatus](#devicetaintrulestatus)
  - [DeviceTaintRuleList](#devicetaintrulelist)
      - [Parameters](#parameters)
      - [Parameters](#parameters)
      - [Parameters](#parameters)
      - [Parameters](#parameters)
      - [Response](#response)
      - [Parameters](#parameters)
      - [Response](#response)
      - [Parameters](#parameters)
      - [Response](#response)
      - [Parameters](#parameters)
      - [Response](#response)
      - [Parameters](#parameters)
      - [Response](#response)
      - [Parameters](#parameters)
      - [Response](#response)
      - [Parameters](#parameters)
  - [Feedback](#feedback)

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
## DeviceTaintRuleSpec
DeviceTaintRuleSpec specifies the selector and one taint.
* **taint** (DeviceTaint), required
The taint that gets applied to matching devices.
*The device this taint is attached to has the "effect" on any claim which does not tolerate the taint and, through the claim, to pods using the claim.*
* **taint.effect** (string), required
The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them.
Valid effects are None, NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here. More effects may get added in the future. Consumers must treat unknown effects like None.
Possible enum values:
* `"NoExecute"` Evict any already-running pods that do not tolerate the device taint.
* `"NoSchedule"` Do not allow new pods to schedule which use a tainted device unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running.
* `"None"` No effect, the taint is purely informational.
* **taint.key** (string), required
The taint key to be applied to a device. Must be a label name.
* **taint.timeAdded** (Time)
TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **taint.value** (string)
The taint value corresponding to the taint key. Must be a label value.
* **deviceSelector** (DeviceTaintSelector)
DeviceSelector defines which device(s) the taint is applied to. All selector criteria must be satisfied for a device to match. The empty selector matches all devices. Without a selector, no devices are matches.
*DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to. The empty selector matches all devices. Without a selector, no devices are matched.*
* **deviceSelector.device** (string)
If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices[].name.
Setting also driver and pool may be required to avoid ambiguity, but is not required.
* **deviceSelector.driver** (string)
If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.
* **deviceSelector.pool** (string)
If pool is set, only devices in that pool are selected.
Also setting the driver name may be useful to avoid ambiguity when different drivers use the same pool name, but this is not required because selecting pools from different drivers may also be useful, for example when drivers with node-local devices use the node name as their pool name.
## DeviceTaintRuleStatus
DeviceTaintRuleStatus provides information about an on-going pod eviction.
* **conditions** ([]Condition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Conditions provide information about the state of the DeviceTaintRule and the cluster at some point in time, in a machine-readable and human-readable format.
The following condition is currently defined as part of this API, more may get added: - Type: EvictionInProgress - Status: True if there are currently pods which need to be evicted, False otherwise
(includes the effects which don't cause eviction).
* Reason: not specified, may change - Message: includes information about number of pending pods and already evicted pods
in a human-readable format, updated periodically, may change
For `effect: None`, the condition above gets set once for each change to the spec, with the message containing information about what would happen if the effect was `NoExecute`. This feedback can be used to decide whether changing the effect to `NoExecute` will work as intended. It only gets set once to avoid having to constantly update the status.
Must have 8 or fewer entries.
*Condition contains details for one aspect of the current state of this API Resource.*
* **conditions.lastTransitionTime** (Time), required
lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.
* **conditions.reason** (string), required
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
* **conditions.status** (string), required
status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
type of condition in CamelCase or in foo.example.com/CamelCase.
* **conditions.observedGeneration** (int64)
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
## DeviceTaintRuleList
DeviceTaintRuleList is a collection of DeviceTaintRules.
* **apiVersion**: resource.k8s.io/v1alpha3
* **kind**: DeviceTaintRuleList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata
* **items** ([][DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)), required
Items is the list of DeviceTaintRules.
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **allowWatchBookmarks** (*in query*): boolean
[allowWatchBookmarks](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#allowWatchBookmarks)
* **continue** (*in query*): string
[continue](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#continue)
* **fieldSelector** (*in query*): string
[fieldSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldSelector)
* **labelSelector** (*in query*): string
[labelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#labelSelector)
* **limit** (*in query*): integer
[limit](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#limit)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **resourceVersion** (*in query*): string
[resourceVersion](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersion)
* **resourceVersionMatch** (*in query*): string
[resourceVersionMatch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersionMatch)
* **sendInitialEvents** (*in query*): boolean
[sendInitialEvents](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#sendInitialEvents)
* **timeoutSeconds** (*in query*): integer
[timeoutSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#timeoutSeconds)
* **watch** (*in query*): boolean
[watch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#watch)
#### Parameters
* **body**: [DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
201 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Created
202 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Accepted
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **body**: [DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
201 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **body**: [DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
201 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **body**: [Patch](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/patch/#Patch), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **force** (*in query*): boolean
[force](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#force)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
201 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **body**: [Patch](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/patch/#Patch), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **force** (*in query*): boolean
[force](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#force)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
201 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **body**: [DeleteOptions](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/delete-options/#DeleteOptions)
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **gracePeriodSeconds** (*in query*): integer
[gracePeriodSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#gracePeriodSeconds)
* **ignoreStoreReadErrorWithClusterBreakingPotential** (*in query*): boolean
[ignoreStoreReadErrorWithClusterBreakingPotential](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#ignoreStoreReadErrorWithClusterBreakingPotential)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **propagationPolicy** (*in query*): string
[propagationPolicy](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#propagationPolicy)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
202 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Accepted
401: Unauthorized
#### Parameters
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
Last modified December 21, 2025 at 5:37 PM PST: [Update resource docs for v1.35 (85b57273c5)](https://github.com/kubernetes/website/commit/85b57273c5db16377a2bec88f72f2cf5b9419524)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [CSINode](docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md)
- [ValidatingAdmissionPolicy](docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md)
- [ServiceAccount](docs-reference-kubernetes-api-authentication-resources-service-account-v1.md)
- [NetworkPolicy](docs-reference-kubernetes-api-policy-resources-network-policy-v1.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
