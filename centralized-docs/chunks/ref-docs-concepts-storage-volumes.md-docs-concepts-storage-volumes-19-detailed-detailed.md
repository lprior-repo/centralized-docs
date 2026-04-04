---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#19-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 817
summary: `apiVersion: v1 kind: Pod metadata: name: rro spec: volumes: - name: mnt hostPath: # tmpfs is mounted on /mnt/tmpfs path: /mnt containers: - name: busybox image: busybox args: [\"sleep\", \"infinity\"]...
---

`apiVersion: v1
kind: Pod
metadata:
name: rro
spec:
volumes:
- name: mnt
hostPath:
# tmpfs is mounted on /mnt/tmpfs
path: /mnt
containers:
- name: busybox
image: busybox
args: ["sleep", "infinity"]
volumeMounts:
# /mnt-rro/tmpfs is not writable
- name: mnt
mountPath: /mnt-rro
readOnly: true
mountPropagation: None
recursiveReadOnly: Enabled
# /mnt-ro/tmpfs is writable
- name: mnt
mountPath: /mnt-ro
readOnly: true
# /mnt-rw/tmpfs is writable
- name: mnt
mountPath: /mnt-rw
`
```
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
Items on this page refer to third party products or projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for those third-party products or projects. See the [CNCF website guidelines](https://github.com/cncf/foundation/blob/main/policies-guidance/website-guidelines.md) for more details.
You should read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before proposing a change that adds an extra third-party link.
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Use an Image Volume With a Pod](docs-tasks-configure-pod-container-image-volumes.md)
- [Managing Service Accounts](docs-reference-access-authn-authz-service-accounts-admin.md)
- [Концепции](ru-docs-concepts.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)