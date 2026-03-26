---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#32-summary
chunk_level: summary
chunk_type: prose
heading: `LeaderElectionConfiguration`
token_count: 110
summary: by another candidate. This is only applicable if leader election is enabled. | |`renewDeadline`**[Required]** [`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|...
---

by another candidate. This is only applicable if leader election is
enabled.
|
|`renewDeadline`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
renewDeadline is the interval between attempts by the acting master to
renew a leadership slot before it stops leading. This must be less
than or equal to the lease duration. This is only applicable if leader
election is enabled.
|
|`retryPeriod`**[Required]**