---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 474
summary: ## Why does the kubelet manage the hosts file? The kubelet manages the `hosts` file for each container of the Pod to prevent the container runtime from modifying the file after the containers have...
---

## Why does the kubelet manage the hosts file?
The kubelet manages the
`hosts` file for each container of the Pod to prevent the container runtime from
modifying the file after the containers have already been started.
Historically, Kubernetes always used Docker Engine as its container runtime, and Docker Engine would
then modify the `/etc/hosts` file after each container had started.
Current Kubernetes can use a variety of container runtimes; even so, the kubelet manages the
hosts file within each container so that the outcome is as intended regardless of which
container runtime you use.
#### Caution:
Avoid making manual changes to the hosts file inside a container.
If you make manual changes to the hosts file,
those changes are lost when the container exits.
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
Last modified October 03, 2024 at 4:50 PM PST: [Removes repeated information (05c1f011d4)](https://github.com/kubernetes/website/commit/05c1f011d49c05d985982dea3eabb7ab68049f9a)
## Related Pages

- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)
- [Access Clusters Using the Kubernetes API](docs-tasks-administer-cluster-access-cluster-api.md)