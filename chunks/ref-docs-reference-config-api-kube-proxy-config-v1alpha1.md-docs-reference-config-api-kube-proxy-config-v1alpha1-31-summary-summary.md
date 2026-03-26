---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#31-summary
chunk_level: summary
chunk_type: prose
heading: `LeaderElectionConfiguration`
token_count: 128
summary: before executing the main loop. Enable this when running replicated components for high availability. | |`leaseDuration`**[Required]**...
---

before executing the main loop. Enable this when running replicated
components for high availability.
|
|`leaseDuration`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
leaseDuration is the duration that non-leader candidates will wait
after observing a leadership renewal until attempting to acquire
leadership of a led but unrenewed leader slot. This is effectively the
maximum duration that a leader can be stopped before it is replaced
by another candidate. This is only applicable if leader election is
enabled.
|
|`renewDeadline`