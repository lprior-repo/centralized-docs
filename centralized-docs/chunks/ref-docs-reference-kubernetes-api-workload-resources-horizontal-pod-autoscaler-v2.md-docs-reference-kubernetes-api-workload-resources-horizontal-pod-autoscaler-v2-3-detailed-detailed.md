---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#3-detailed
chunk_level: detailed
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 997
summary: * **maxReplicas** (int32), required maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas. * **scaleTargetRef**...
---

* **maxReplicas** (int32), required
maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
* **scaleTargetRef** (CrossVersionObjectReference), required
scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
*CrossVersionObjectReference contains enough information to let you identify the referred resource.*
* **scaleTargetRef.kind** (string), required
kind is the kind of the referent; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **scaleTargetRef.name** (string), required
name is the name of the referent; More info: [https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names)
* **scaleTargetRef.apiVersion** (string)
apiVersion is the API version of the referent
* **minReplicas** (int32)
minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down. It defaults to 1 pod. minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured. Scaling is active as long as at least one metric value is available.
* **behavior** (HorizontalPodAutoscalerBehavior)
behavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively). If not set, the default HPAScalingRules for scale up and scale down are used.
*HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).*
* **behavior.scaleDown** (HPAScalingRules)
scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
\*HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance.
Scaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
The tolerance is applied to the metric values and prevents scaling too eagerly for small metric variations. (Note that setting a tolerance requires the beta HPAConfigurableTolerance feature gate to be enabled.)\*
* **behavior.scaleDown.policies** ([]HPAScalingPolicy)
*Atomic: will be replaced during a merge*
policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.
*HPAScalingPolicy is a single policy which must hold true for a specified past interval.*
* **behavior.scaleDown.policies.type** (string), required
type is used to specify the scaling policy.
* **behavior.scaleDown.policies.value** (int32), required
value contains the amount of change which is permitted by the policy. It must be greater than zero
* **behavior.scaleDown.policies.periodSeconds** (int32), required
periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
* **behavior.scaleDown.selectPolicy** (string)
selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
* **behavior.scaleDown.stabilizationWindowSeconds** (int32)
stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).