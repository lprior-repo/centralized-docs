---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#5-standard
chunk_level: standard
chunk_type: table
heading: `LeaderElectionConfiguration`
token_count: 491
summary: ## `LeaderElectionConfiguration` **Appears in:** * [KubeSchedulerConfiguration](#kubescheduler-config-k8s-io-v1-KubeSchedulerConfiguration) *...
---

## `LeaderElectionConfiguration`
**Appears in:**
* [KubeSchedulerConfiguration](#kubescheduler-config-k8s-io-v1-KubeSchedulerConfiguration)
* [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration)
LeaderElectionConfiguration defines the configuration of leader election
clients for components that can run with leader election enabled.
|Field|Description|
|`leaderElect`**[Required]**
`bool`|
leaderElect enables a leader election client to gain leadership
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
|`renewDeadline`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
renewDeadline is the interval between attempts by the acting master to
renew a leadership slot before it stops leading. This must be less
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
|`resourceName`**[Required]**
`string`|
resourceName indicates the name of resource object that will be used to lock
during leader election cycles.
|
|`resourceNamespace`**[Required]**
`string`|
resourceName indicates the namespace of resource object that will be used to lock
during leader election cycles.
|