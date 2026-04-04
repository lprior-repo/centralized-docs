---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#4-detailed
chunk_level: detailed
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 921
summary: * **behavior.scaleDown.policies.periodSeconds** (int32), required periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less...
---

* **behavior.scaleDown.policies.periodSeconds** (int32), required
periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
* **behavior.scaleDown.selectPolicy** (string)
selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
* **behavior.scaleDown.stabilizationWindowSeconds** (int32)
stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
* **behavior.scaleDown.tolerance** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
tolerance is the tolerance on the ratio between the current and desired metric value under which no updates are made to the desired number of replicas (e.g. 0.01 for 1%). Must be greater than or equal to zero. If not set, the default cluster-wide tolerance is applied (by default 10%).
For example, if autoscaling is configured with a memory consumption target of 100Mi, and scale-down and scale-up tolerances of 5% and 1% respectively, scaling will be triggered when the actual consumption falls below 95Mi or exceeds 101Mi.
This is an beta field and requires the HPAConfigurableTolerance feature gate to be enabled.
* **behavior.scaleUp** (HPAScalingRules)
scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:
* increase no more than 4 pods per 60 seconds
* double the number of pods per 60 seconds
No stabilization is used.
\*HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance.
Scaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
The tolerance is applied to the metric values and prevents scaling too eagerly for small metric variations. (Note that setting a tolerance requires the beta HPAConfigurableTolerance feature gate to be enabled.)\*
* **behavior.scaleUp.policies** ([]HPAScalingPolicy)
*Atomic: will be replaced during a merge*
policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.
*HPAScalingPolicy is a single policy which must hold true for a specified past interval.*
* **behavior.scaleUp.policies.type** (string), required
type is used to specify the scaling policy.
* **behavior.scaleUp.policies.value** (int32), required
value contains the amount of change which is permitted by the policy. It must be greater than zero
* **behavior.scaleUp.policies.periodSeconds** (int32), required
periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
* **behavior.scaleUp.selectPolicy** (string)
selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
* **behavior.scaleUp.stabilizationWindowSeconds** (int32)
stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).