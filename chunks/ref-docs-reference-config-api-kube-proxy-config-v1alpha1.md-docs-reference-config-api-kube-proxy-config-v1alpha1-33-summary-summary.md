---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#33-summary
chunk_level: summary
chunk_type: prose
heading: `LeaderElectionConfiguration`
token_count: 126
summary: than or equal to the lease duration. This is only applicable if leader election is enabled. | |`retryPeriod`**[Required]**...
---

than or equal to the lease duration. This is only applicable if leader
election is enabled.
|
|`retryPeriod`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
retryPeriod is the duration the clients should wait between attempting
acquisition and renewal of a leadership. This is only applicable if
leader election is enabled.
|
|`resourceLock`**[Required]**
`string`|
resourceLock indicates the resource object type that will be used to lock
during leader election cycles.
|
|`resourceName`