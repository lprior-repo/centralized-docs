---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#2-standard
chunk_level: standard
chunk_type: table
heading: Related Pages
token_count: 505
summary: ## `CommandOptionDefault` **Appears in:** * [AliasOverride](#kubectl-config-k8s-io-v1alpha1-AliasOverride) * [CommandDefaults](#kubectl-config-k8s-io-v1alpha1-CommandDefaults) CommandOptionDefault...
---

## `CommandOptionDefault`
**Appears in:**
* [AliasOverride](#kubectl-config-k8s-io-v1alpha1-AliasOverride)
* [CommandDefaults](#kubectl-config-k8s-io-v1alpha1-CommandDefaults)
CommandOptionDefault stores the name and the specified default
value of an option.
|Field|Description|
|`name`**[Required]**
`string`|
Flag name (long form, without dashes).
|
|`default`**[Required]**
`string`|
In a string format of a default value. It will be parsed
by kubectl to the compatible value of the flag.
|
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
Last modified September 04, 2025 at 5:02 PM PST: [Config API for v1.34 (3557e3070d)](https://github.com/kubernetes/website/commit/3557e3070dcd5659f259e302b53f98adfd9a79f1)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)