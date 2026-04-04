---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#6-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 446
summary: * **behavior.scaleUp** (HPAScalingRules) scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of: * increase no more than 4 pods per 60 seconds * double the number of...
---

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