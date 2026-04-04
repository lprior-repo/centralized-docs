---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#5-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 409
summary: ## Observability The metric `scheduler\_pending\_pods` comes with a new label `\"gated\"` to distinguish whether a Pod has been tried scheduling but claimed as unschedulable, or explicitly marked as...
---

## Observability
The metric `scheduler\_pending\_pods` comes with a new label `"gated"` to distinguish whether a Pod
has been tried scheduling but claimed as unschedulable, or explicitly marked as not ready for
scheduling. You can use `scheduler\_pending\_pods{queue="gated"}` to check the metric result.
## Mutable Pod scheduling directives
You can mutate scheduling directives of Pods while they have scheduling gates, with certain constraints.
At a high level, you can only tighten the scheduling directives of a Pod. In other words, the updated
directives would cause the Pods to only be able to be scheduled on a subset of the nodes that it would
previously match. More concretely, the rules for updating a Pod's scheduling directives are as follows:
1. For `.spec.nodeSelector`, only additions are allowed. If absent, it will be allowed to be set.
2. For `spec.affinity.nodeAffinity`, if nil, then setting anything is allowed.
3. If `NodeSelectorTerms` was empty, it will be allowed to be set.
If not empty, then only additions of `NodeSelectorRequirements` to `matchExpressions`
or `fieldExpressions` are allowed, and no changes to existing `matchExpressions`
and `fieldExpressions` will be allowed. This is because the terms in
`.requiredDuringSchedulingIgnoredDuringExecution.NodeSelectorTerms`, are ORed
while the expressions in `nodeSelectorTerms[].matchExpressions` and
`nodeSelectorTerms[].fieldExpressions` are ANDed.
4. For `.preferredDuringSchedulingIgnoredDuringExecution`, all updates are allowed.
This is because preferred terms are not authoritative, and so policy controllers
don't validate those terms.
## What's next
* Read the [PodSchedulingReadiness KEP](https://github.com/kubernetes/enhancements/blob/master/keps/sig-scheduling/3521-pod-scheduling-readiness) for more details