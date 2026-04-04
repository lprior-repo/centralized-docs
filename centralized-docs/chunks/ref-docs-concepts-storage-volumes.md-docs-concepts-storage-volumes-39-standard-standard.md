---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#39-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 490
summary: When this property is recognized by kubelet and kube-apiserver, the `.status.containerStatuses[\*].volumeMounts[\*].recursiveReadOnly` field is set to either `Enabled` or `Disabled`. ####...
---

When this property is recognized by kubelet and kube-apiserver,
the `.status.containerStatuses[\*].volumeMounts[\*].recursiveReadOnly` field is set to either
`Enabled` or `Disabled`.
#### Implementations
**Note:** This section links to third party projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for these projects, which are listed alphabetically. To add a project to this list, read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before submitting a change. [More information.](#third-party-content-disclaimer)
The following container runtimes are known to support recursive read-only mounts.
CRI-level:
* [containerd](https://containerd.io/), since v2.0
* [CRI-O](https://cri-o.io/), since v1.30
OCI-level:
* [runc](https://runc.io/), since v1.1
* [crun](https://github.com/containers/crun), since v1.8.6## What's next
Follow an example of [deploying WordPress and MySQL with Persistent Volumes](/docs/tutorials/stateful-application/mysql-wordpress-persistent-volume/).
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
Last modified March 26, 2026 at 5:41 PM PST: [chore: Terminology, grammar, deprecations in volumes.md (a724916000)](https://github.com/kubernetes/website/commit/a724916000ffaa99e05d910efc4f9d3189cd0585)
Items on this page refer to third party products or projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for those third-party products or projects.